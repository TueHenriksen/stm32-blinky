# STM32-blinky

This hello world application is for STM32F746ZGTx and will just blink an LED. 
This is based on the great work of [Jonathan Klimt]. 

The target platform is set in `.cargo/config`, so you don't have to specify 
each time you run cargo.

The file `.memory.x` is used to specify the memory layout of the device. This
is used by the linker to place the code in the correct memory location. This is 
handled by the run-time.

To flash the device I use [cargo-flash]. After it has been installed you can 
use the following command to flash the device:

```bash
cargo flash --chip stm32f746zgtx --release
```

[jonathan klimt]: https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
[cargo-flash]: https://probe.rs/docs/tools/cargo-flash/