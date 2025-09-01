//! https://github.com/stm32-rs/stm32f1xx-hal/blob/v0.7.0/examples/blinky.rs

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use split_hover_esc as _; // global logger + panicking-behavior + memory layout

use defmt::{info, trace}; // export DEFMT_LOG=trace in shell

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    trace!("Get access to the core peripherals from the cortex-m crate");
    let cp = cortex_m::Peripherals::take().unwrap();

    trace!("Get access to the device specific peripherals from the peripheral access crate");
    let dp = pac::Peripherals::take().unwrap();

    trace!("Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs");
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    trace!("Freeze the configuration of all the clocks in the system and store the frozen frequencies in clocks");
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    trace!("Acquire the GPIOC peripheral");
    let mut gpioc = dp.GPIOC.split();

    trace!("Configure gpio C pin 13 as a push-pull output.");
    trace!("The `crh` register is passed to the function in order to configure the port.");
    trace!("For pins 0-7, crl should be passed instead.");
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    trace!("Configure the syst timer to trigger an update every second");
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    trace!("Wait for the timer to trigger an update and change the state of the LED");
    loop {
        trace!("waiting");
        block!(timer.wait()).unwrap();

        info!("set led high");
        led.set_high();

        trace!("waiting again");
        block!(timer.wait()).unwrap();

        info!("set led low");
        led.set_low();
    }
}
