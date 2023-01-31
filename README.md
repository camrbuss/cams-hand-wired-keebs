# cams-hand-wired-keebs
Firmware and other artifacts of building hand wired keyboards

## Pteron 38
[Design](https://github.com/FSund/pteron-keyboard) by FSund
![Pteron38](img/pteron38.jpg)
![Pteron38 Wiring](img/pteron38_wiring.jpg)

## Lumberjack Remix
[Design](https://github.com/peej/lumberjack-keyboard) by peej
![lumberjack](img/lumberjack.jpg)

## Skeletyl
[Design](https://github.com/Bastardkb/Skeletyl) by Bastardkb
![skeletyl](img/skeletyl.jpg)
![skeletyl](img/skeletyl_1_0.jpg)

## Flashing
```
cargo objcopy --release -- -O binary binary.bin
dfu-util -a 0 -s 0x08000000:leave -D binary.bin --reset
```
