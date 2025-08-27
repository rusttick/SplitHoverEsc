#![no_main]
#![no_std]

use split_hover_esc as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    split_hover_esc::exit()
}
