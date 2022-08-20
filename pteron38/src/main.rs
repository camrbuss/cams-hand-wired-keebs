#![no_main]
#![no_std]

use panic_halt as _;
use stm32f4xx_hal as hal;

mod layout;

#[rtic::app(device = crate::hal::pac, peripherals = true)]
mod app {
    use hal::gpio::{self, ErasedPin, Input, Output, PullUp, PushPull};
    use hal::otg_fs::{UsbBusType, USB};
    use hal::prelude::*;
    use hal::{pac, timer};
    use keyberon::debounce::Debouncer;
    use keyberon::key_code::KbHidReport;
    use keyberon::layout;
    use keyberon::matrix::Matrix;
    use stm32f4xx_hal as hal;
    use usb_device::bus::UsbBusAllocator;
    use usb_device::class::UsbClass as _;

    use crate::layout::CustomActions;

    type UsbClass = keyberon::Class<'static, UsbBusType, Leds>;
    type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;
    static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

    pub struct Leds {
        caps_lock: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
    }
    impl keyberon::keyboard::Leds for Leds {
        fn caps_lock(&mut self, status: bool) {
            if status {
                self.caps_lock.set_low()
            } else {
                self.caps_lock.set_high()
            }
        }
    }

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
    }

    #[local]
    struct Local {
        matrix: Matrix<ErasedPin<Input<PullUp>>, ErasedPin<Output<PushPull>>, 10, 4>,
        debouncer: Debouncer<[[bool; 10]; 4]>,
        chording: keyberon::chording::Chording<3_usize>,
        timer: timer::CountDownTimer<pac::TIM3>,
        watchdog: hal::watchdog::IndependentWatchdog,
        layout: layout::Layout<10, 4, 8, CustomActions>,
    }

    #[init]
    fn init(c: init::Context) -> (Shared, Local, init::Monotonics) {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];

        let rcc = c.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .require_pll48clk()
            .freeze();
        let gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        let gpioc = c.device.GPIOC.split();

        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();
        let leds = Leds { caps_lock: led };

        let usb = USB {
            usb_global: c.device.OTG_FS_GLOBAL,
            usb_device: c.device.OTG_FS_DEVICE,
            usb_pwrclk: c.device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        unsafe {
            USB_BUS = Some(UsbBusType::new(usb, &mut EP_MEMORY));
        }
        let usb_class = keyberon::new_class(unsafe { USB_BUS.as_ref().unwrap() }, leds);
        let usb_dev = keyberon::new_device(unsafe { USB_BUS.as_ref().unwrap() });

        let mut timer = timer::Timer::new(c.device.TIM3, &clocks).start_count_down(1000.hz());
        timer.listen(timer::Event::TimeOut);

        let matrix = Matrix::new(
            [
                gpioa.pa6.into_pull_up_input().erase(),
                gpioa.pa7.into_pull_up_input().erase(),
                gpiob.pb0.into_pull_up_input().erase(),
                gpiob.pb1.into_pull_up_input().erase(),
                gpioa.pa2.into_pull_up_input().erase(),
                gpiob.pb14.into_pull_up_input().erase(),
                gpiob.pb15.into_pull_up_input().erase(),
                gpioa.pa5.into_pull_up_input().erase(),
                gpioa.pa4.into_pull_up_input().erase(),
                gpioa.pa3.into_pull_up_input().erase(),
            ],
            [
                gpiob.pb10.into_push_pull_output().erase(),
                gpioa.pa1.into_push_pull_output().erase(),
                gpioa.pa0.into_push_pull_output().erase(),
                gpiob.pb9.into_push_pull_output().erase(),
            ],
        );

        let mut watchdog = hal::watchdog::IndependentWatchdog::new(c.device.IWDG);
        watchdog.start(hal::time::MilliSeconds(10));

        (
            Shared { usb_dev, usb_class },
            Local {
                timer,
                watchdog,
                debouncer: Debouncer::new([[false; 10]; 4], [[false; 10]; 4], 15),
                chording: keyberon::chording::Chording::new(&crate::layout::CHORDS),
                matrix: matrix.unwrap(),
                layout: keyberon::layout::Layout::new(&crate::layout::LAYERS),
            },
            init::Monotonics(),
        )
    }

    #[task(binds = OTG_FS, priority = 2, shared = [usb_dev, usb_class])]
    fn usb_tx(c: usb_tx::Context) {
        (c.shared.usb_dev, c.shared.usb_class).lock(|u, k| {
            if u.poll(&mut [k]) {
                k.poll();
            }
        });
    }

    #[task(binds = OTG_FS_WKUP, priority = 2, shared = [usb_dev, usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        (c.shared.usb_dev, c.shared.usb_class).lock(|u, k| {
            if u.poll(&mut [k]) {
                k.poll();
            }
        });
    }

    #[task(binds = TIM3, priority = 1, shared = [usb_class], local = [layout, matrix, debouncer, chording, timer, watchdog])]
    fn tick(c: tick::Context) {
        c.local.timer.clear_interrupt(timer::Event::TimeOut);
        c.local.watchdog.feed();

        for event in c.local.chording.tick(
            c.local
                .debouncer
                .events(c.local.matrix.get().unwrap())
                .collect(),
        ) {
            c.local.layout.event(event);
        }
        match c.local.layout.tick() {
            layout::CustomEvent::Press(event) => match event {
                CustomActions::Bootload => unsafe { cortex_m::asm::bootload(0x1FFF0000 as _) },
                CustomActions::Reset => {
                    cortex_m::peripheral::SCB::sys_reset();
                }
            },
            _ => (),
        }
        let mut usb_class = c.shared.usb_class;
        let report: KbHidReport = c.local.layout.keycodes().collect();
        if usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
            while let Ok(0) = usb_class.lock(|k| k.write(report.as_bytes())) {}
        }
    }
}
