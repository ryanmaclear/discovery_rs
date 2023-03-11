#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use heapless::Vec;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let byte = nb::block!(serial.read()).unwrap();
            nb::block!(serial.write(byte)).unwrap();
            nb::block!(serial.flush()).unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "\r\nerror: buffer full\r\n").unwrap();
            }

            if byte == 13 {
                write!(serial, "\r\n").unwrap();
                for byte in buffer.iter().rev().chain(&[b'\n',b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }
        }

        nb::block!(serial.flush()).unwrap();
    }
}
