#![no_std]
#![no_main]

use panic_halt as _;

mod layout;

#[rtic::app(device = sparkfun_pro_micro_rp2040::hal::pac, peripherals = true, dispatchers = [PIO0_IRQ_0])]
mod app {
    use cortex_m::prelude::{
        _embedded_hal_watchdog_Watchdog, _embedded_hal_watchdog_WatchdogEnable,
    };
    use embedded_time::{duration::units::*, rate::*};
    use keyberon::chording::Chording;
    use keyberon::debounce::Debouncer;
    use keyberon::key_code::KbHidReport;
    use keyberon::layout::{self, Layout};
    use keyberon::matrix::Matrix;
    use sparkfun_pro_micro_rp2040::{
        hal::{
            self,
            clocks::{Clock, ClockSource, ClocksManager, InitError},
            gpio::DynPin,
            pll::{common_configs::PLL_USB_48MHZ, setup_pll_blocking},
            sio::Sio,
            timer::Alarm,
            usb::UsbBus,
            watchdog::Watchdog,
            xosc::setup_xosc_blocking,
        },
        Pins, XOSC_CRYSTAL_FREQ,
    };
    use usb_device::class_prelude::*;

    use crate::layout::CustomActions;

    const SCAN_TIME_US: u32 = 1000;

    #[shared]
    struct Shared {
        usb_dev: usb_device::device::UsbDevice<'static, rp2040_hal::usb::UsbBus>,
        usb_class: keyberon::hid::HidClass<
            'static,
            rp2040_hal::usb::UsbBus,
            keyberon::keyboard::Keyboard<()>,
        >,
    }

    #[local]
    struct Local {
        watchdog: hal::watchdog::Watchdog,
        chording: Chording<6>,
        matrix: Matrix<DynPin, DynPin, 14, 3>,
        debouncer: Debouncer<[[bool; 14]; 3]>,
        alarm: hal::timer::Alarm0,
        layout: Layout<14, 3, 5, CustomActions>,
    }

    #[init(local = [bus: Option<UsbBusAllocator<hal::usb::UsbBus>> = None])]
    fn init(c: init::Context) -> (Shared, Local, init::Monotonics) {
        // Soft-reset does not release the hardware spinlocks
        // Release them now to avoid a deadlock after debug or watchdog reset
        unsafe {
            hal::sio::spinlock_reset();
        }
        let mut resets = c.device.RESETS;
        let mut watchdog = Watchdog::new(c.device.WATCHDOG);

        // Enable the xosc
        let xosc = setup_xosc_blocking(c.device.XOSC, XOSC_CRYSTAL_FREQ.Hz())
            .map_err(InitError::XoscErr)
            .ok()
            .unwrap();

        // Start tick in watchdog
        watchdog.enable_tick_generation((XOSC_CRYSTAL_FREQ / 1_000_000) as u8);

        let mut clocks = ClocksManager::new(c.device.CLOCKS);

        // Configure PLLs
        //                   REF     FBDIV VCO            POSTDIV
        // PLL SYS: 12 / 1 = 12MHz * 125 = 1500MHZ / 6 / 4 = 62.5MHz
        // PLL USB: 12 / 1 = 12MHz * 40  = 480 MHz / 5 / 2 =  48MHz
        let pll_sys = setup_pll_blocking(
            c.device.PLL_SYS,
            xosc.operating_frequency().into(),
            hal::pll::PLLConfig {
                vco_freq: embedded_time::rate::Megahertz(1500),
                refdiv: 1,
                post_div1: 6,
                post_div2: 4,
            },
            &mut clocks,
            &mut resets,
        )
        .map_err(InitError::PllError)
        .ok()
        .unwrap();
        let pll_usb = setup_pll_blocking(
            c.device.PLL_USB,
            xosc.operating_frequency().into(),
            PLL_USB_48MHZ,
            &mut clocks,
            &mut resets,
        )
        .map_err(InitError::PllError)
        .ok()
        .unwrap();

        // Configure clocks
        // CLK_REF = XOSC (12MHz) / 1 = 12MHz
        clocks
            .reference_clock
            .configure_clock(&xosc, xosc.get_freq())
            .map_err(InitError::ClockError)
            .ok()
            .unwrap();

        // CLK SYS = PLL SYS (62.5 MHz) / 1 = 62.5MHz
        clocks
            .system_clock
            .configure_clock(&pll_sys, pll_sys.get_freq())
            .map_err(InitError::ClockError)
            .ok()
            .unwrap();

        // CLK USB = PLL USB (48MHz) / 1 = 48MHz
        clocks
            .usb_clock
            .configure_clock(&pll_usb, pll_usb.get_freq())
            .map_err(InitError::ClockError)
            .ok()
            .unwrap();

        // CLK ADC = PLL USB (48MHZ) / 1 = 48MHz
        clocks
            .adc_clock
            .configure_clock(&pll_usb, pll_usb.get_freq())
            .map_err(InitError::ClockError)
            .ok()
            .unwrap();

        // CLK RTC = PLL USB (48MHz) / 1024 = 46875Hz
        clocks
            .rtc_clock
            .configure_clock(&pll_usb, 46875u32.Hz())
            .map_err(InitError::ClockError)
            .ok()
            .unwrap();

        // CLK PERI = clk_sys. Used as reference clock for Peripherals. No dividers so just select and enable
        // Normally choose clk_sys or clk_usb
        clocks
            .peripheral_clock
            .configure_clock(&clocks.system_clock, clocks.system_clock.freq())
            .map_err(InitError::ClockError)
            .ok()
            .unwrap();

        let sio = Sio::new(c.device.SIO);
        let pins = Pins::new(
            c.device.IO_BANK0,
            c.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        let matrix: Matrix<DynPin, DynPin, 14, 3> = Matrix::new(
            [
                pins.adc1.into_pull_up_input().into(),
                pins.adc0.into_pull_up_input().into(),
                pins.adc2.into_pull_up_input().into(),
                pins.cipo.into_pull_up_input().into(),
                pins.sck.into_pull_up_input().into(),
                pins.adc3.into_pull_up_input().into(),
                pins.gpio6.into_pull_up_input().into(),
                pins.gpio2.into_pull_up_input().into(),
                pins.gpio5.into_pull_up_input().into(),
                pins.tx0.into_pull_up_input().into(),
                pins.rx0.into_pull_up_input().into(),
                pins.gpio4.into_pull_up_input().into(),
                pins.copi.into_pull_up_input().into(), // not wired
                pins.ncs.into_pull_up_input().into(),  // not wired
            ],
            [
                pins.rx1.into_push_pull_output().into(),
                pins.gpio7.into_push_pull_output().into(),
                pins.gpio3.into_push_pull_output().into(),
            ],
        )
        .unwrap();

        let layout = Layout::new(&crate::layout::LAYERS);
        let debouncer = Debouncer::new([[false; 14]; 3], [[false; 14]; 3], 20);

        let chording = Chording::new(&crate::layout::CHORDS);

        let mut timer = hal::Timer::new(c.device.TIMER, &mut resets);
        let mut alarm = timer.alarm_0().unwrap();
        let _ = alarm.schedule(SCAN_TIME_US.microseconds());
        alarm.enable_interrupt();

        *c.local.bus = Some(UsbBusAllocator::new(UsbBus::new(
            c.device.USBCTRL_REGS,
            c.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut resets,
        )));
        let usb_bus = c.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, ());
        let usb_dev = keyberon::new_device(usb_bus);

        // Start watchdog and feed it with the lowest priority task at 1000hz
        watchdog.start(10_000.microseconds());

        (
            Shared { usb_dev, usb_class },
            Local {
                layout,
                alarm,
                chording,
                watchdog,
                matrix,
                debouncer,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBCTRL_IRQ, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        (c.shared.usb_dev, c.shared.usb_class).lock(|u, k| {
            if u.poll(&mut [k]) {
                k.poll();
            }
        });
    }

    #[task(
        binds = TIMER_IRQ_0,
        priority = 1,
        shared = [usb_class],
        local = [layout, matrix, debouncer, chording, watchdog, alarm],
    )]
    fn scan_timer_irq(c: scan_timer_irq::Context) {
        let alarm = c.local.alarm;
        alarm.clear_interrupt();
        let _ = alarm.schedule(SCAN_TIME_US.microseconds());

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
                CustomActions::Bootload => hal::rom_data::reset_to_usb_boot(0, 0),
                CustomActions::Reset => cortex_m::peripheral::SCB::sys_reset(),
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
