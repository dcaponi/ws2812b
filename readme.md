# Getting Started

1. install cargo generate `cargo install cargo-generate`
2. run `cargo generate --git https://github.com/dcaponi/esp32-rust-template`
3. ensure the project builds & links `cargo build`
4. Now quit f*cking around and honk a bunch of one bits to your device -BCR

# Other Things You Might Need
## Linux (Ubuntu)

*Low Level Build & Flash tools* 
```
sudo apt-get install -y clang llvm flex bison gperf cmake
ninja-build ccache libffi-devlibssl-dev libusb-1.0-0 libudev-dev
dfu-util package-config
 ```

*espflash, ldrpoxy* `cargo install espflash ldproxy`

*llvm tools preview* `rustup component add llvm-tools-preview`

*Add the risc-v target for esp32 devices* `rustup target add riscv32imc-unknown-none-elf`
