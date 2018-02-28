#![feature(never_type)]
#![feature(used)]
#![no_std]

extern crate hifive;
#[macro_use]
extern crate nb;
#[macro_use]
extern crate riscv_semihosting;
#[macro_use]
extern crate stlog;

mod log;
mod stepper;

use hifive::prelude::*;
use hifive::{Clint, UExt};

set_global_logger!(log::JtagLogger);

fn main() {
    let p = hifive::init(115_200);
    let uart_logger = log::UartLogger::new(p.UART0);
    info!("openocd: Hello world!");
    info!(uart_logger, "uart: Hello world!").unwrap();

    stepper::init(p.GPIO0);
    stepper::set_direction(p.GPIO0, stepper::Direction::Forward);

    let timer = Clint(p.CLINT);
    timer.set_timeout(400.us());

    loop {
        stepper::half_step(p.GPIO0);
        block!(timer.wait()).unwrap();
        timer.restart();
    }
}
