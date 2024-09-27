#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{OutputPin, PinState}};
use microbit::{hal::Timer, pac::TIMER0, Board};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board: Board = Board::take().unwrap();
    let mut timer: Timer<TIMER0> = Timer::new(board.TIMER0);
    let blink_delay_ms: u32 = 300_u32;

    board.display_pins.col3.set_state(PinState::Low).unwrap();

    loop {
        board.display_pins.row3.set_high().unwrap();
        timer.delay_ms(blink_delay_ms);
        board.display_pins.row3.set_low().unwrap();
        timer.delay_ms(blink_delay_ms);
    }
}
