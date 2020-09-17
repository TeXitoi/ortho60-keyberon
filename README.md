# ortho60-keyberon [![Build status](https://travis-ci.org/TeXitoi/ortho60-keyberon.svg?branch=master)](https://travis-ci.org/TeXitoi/ortho60-keyberon)

Keyberon port to the [Cannon Keys Ortho60](https://cannonkeys.com/collections/frontpage/products/ortho60)

## Installing the precompiled firmware

Download the [precompiled firmware](ortho60-keyberon.bin) and install
it as described in the [official
documentation](https://docs.cannonkeys.com/flashing/).

Sort version: while the keyboard is connected to USB, press the reset
button on the blue pill.  The green LED should blink. In a terminal, type

```shell
sudo dfu-util -d 1eaf:0003 -a 2 -D ortho60-keyberon.bin
```

Push reset and you're ready to go.

## Installing a custom firmware

First install the rust toolchain by typing, as user, these 4 commands:

```shell
curl https://sh.rustup.rs -sSf | sh
rustup target add thumbv7m-none-eabi
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

Then build:

```shell
git clone https://github.com/TeXitoi/ortho60-keyberon
cd ortho60-keyberon
# edit src/layout.rs to customize your layout
cargo objcopy --bin ortho60-keyberon --release -- -O binary ortho60-keyberon.bin
```

Now flash the generated `ortho60-keyberon.bin` file by following the instructions in the previous section.

## Notes

reflash bootloader:

```shell
openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "init; reset halt; stm32f1x mass_erase 0; program generic_boot20_pc13.bin exit 0x08000000"
```

Cannonkeys flash procedure: https://docs.cannonkeys.com/flashing/

## References

- https://github.com/TeXitoi/keyberon
- https://github.com/TeXitoi/blue-pill-quickstart
- https://cannonkeys.com
