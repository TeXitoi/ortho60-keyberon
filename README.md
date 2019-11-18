# ortho60-keyberon [![Build status](https://travis-ci.org/TeXitoi/ortho60-keyberon.svg?branch=master)](https://travis-ci.org/TeXitoi/ortho60-keyberon)

Keyberon port to the [Canon Keys Ortho60](https://cannonkeys.com/collections/frontpage/products/ortho60)

This is a work in progress. Currently, it should be functionnal without bootloader. I don't have the hardware, so not tested.

## Installing the precompiled firmware

Download the [precompiled firmware](ortho60-keyberon.bin).

Install openocd:

```shell
sudo apt-get install openocd
```

Plug the ST-Link to the blue pill.

Open a shell in the directory where you downloaded the firmware and type:

```shell
openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "program ortho60-keyberon.bin exit 0x08000000"
```

## Installing a custom firmware

First install the rust toolchain:

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
# edit src/main.rs to customize keymap
cargo objcopy --bin ortho60-keyberon --release -- -O binary ortho60-keyberon.bin
```

Now flash the generated `ortho60-keyberon.bin` file by following the instructions in the previous section.

You can also compile and flash using `cargo run --release`. See https://github.com/TeXitoi/blue-pill-quickstart

## Notes

reflash bootloader:

```shell
openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "init; reset halt; stm32f1x mass_erase 0; program generic_boot20_pc13.bin exit 0x08000000"
```

flash with dfu:

```shell
sudo dfu-util -d 1eaf:0003 -a 2 -D ortho60-keyberon.bin
```

Cannonkeys flash proceddure: https://docs.cannonkeys.com/flashing/

## References

- https://github.com/TeXitoi/keyberon
- https://github.com/TeXitoi/blue-pill-quickstart
- https://cannonkeys.com
