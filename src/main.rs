// std and main are not available for bare metal software
#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;

use cortex_m_rt::entry;
use stm32f7xx_hal::{pac, prelude::*};

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Get handles to the hardware objects. These functions can only be called
    // once, so that the borrowchecker can ensure you don't reconfigure
    // something by accident.
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // GPIO pins on the STM32F1 must be driven by the APB2 peripheral clock.
    // This must be enabled first. The HAL provides some abstractions for
    // us: First get a handle to the RCC peripheral:
    let rcc = dp.RCC.constrain();

    // The GPIOB can be enabled by calling the `split` function.
    let gpiob = dp.GPIOB.split();

    // This gives us an exclusive handle to the GPIOB peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin B7
    // is connected to the blue onboard LED LD2.
    let mut led = gpiob.pb7.into_push_pull_output();

    // Now we need a delay object. The delay is of course depending on the clock
    // frequency of the microcontroller, so we need to fix the frequency
    // first. We set the controllers frequency to 216 MHz:
    let clocks = rcc.cfgr.sysclk(216.MHz()).freeze();

    // The `clocks` handle ensures that the clocks are now configured and gives
    // the `Delay::new` function access to the configured frequency. With
    // this information it can later calculate how many cycles it has to
    // wait. The function also consumes the System Timer peripheral, so that no
    // other function can access it. Otherwise the timer could be reset during a
    // delay.
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(100_u16);
        led.set_low();
        delay.delay_ms(400_u16);
    }
}
