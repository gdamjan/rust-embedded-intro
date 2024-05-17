#![no_main]
#![no_std]

// Some panic handler needs to be included. This one halts the processor on panic.
use panic_halt as _;

// Use `main` as the entry point of this application, which may never return.
#[cortex_m_rt::entry]
fn main() -> ! {
    // initialization

    loop {
        // application logic
    }
}
