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

set_global_logger!(log::JtagLogger);

fn main() {
    let peripherals = hifive::init(115_200);
    let uart_logger = log::UartLogger::new(peripherals.UART0);
    info!("openocd: Hello world!");
    info!(uart_logger, "uart: Hello world!").unwrap();
}
