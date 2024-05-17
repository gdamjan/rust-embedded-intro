#![no_main]
#![no_std]

// Some panic handler needs to be included. This one halts the processor on panic.
use panic_halt as _;

// include cortex_m for the critical_section implementation required by rtt_target
use cortex_m as _;

use rtt_target::{rprintln, rtt_init_print};

// Use `main` as the entry point of this application, which may never return.
#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        rprintln!("Hello, Universe!");
    }
}
