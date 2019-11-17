# ortho60-keyberon

Keyberon port to the [Canon Keys Ortho60](https://cannonkeys.com/collections/frontpage/products/ortho60)

## Installing this firmware

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
cargo objcopy --bin ortho60-keyberon --release -- -O binary ortho60-keyberon.bin
```

Now flash the generated `ortho60-keyberon.bin` file by following https://docs.cannonkeys.com/flashing/

## Tricks

reflash black bootloader:

```shell
openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "init; reset halt; stm32f1x mass_erase 0; program generic_boot20_pc13.bin exit 0x08000000"
```

flash with dfu:

```shell
sudo dfu-util -d 1eaf:0003 -a 2 -D ortho60-keyberon.bin
```

## References

- https://github.com/TeXitoi/keyberon
- https://github.com/TeXitoi/blue-pill-quickstart
- https://cannonkeys.com
