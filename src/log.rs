use hifive::prelude::*;
use hifive::Serial;
use hifive::e310x::UART0;
use stlog;

/// JTAG Logger implementation
pub struct JtagLogger;

impl stlog::Logger for JtagLogger {
    type Error = !;

    fn log(&self, addr: u8) -> Result<(), !> {
        const STDOUT: usize = 1;
        unsafe { syscall!(WRITE, STDOUT, *&addr, 1) };
        Ok(())
    }
}

/// UART Logger implementation
pub struct UartLogger {
    uart: &'static UART0
}

impl UartLogger {
    pub fn new(uart: &'static UART0) -> Self {
        UartLogger { uart }
    }
}

impl stlog::Logger for UartLogger {
    // required: the error type must be `!`
    type Error = !;

    fn log(&self, addr: u8) -> Result<(), !> {
        let serial = Serial(self.uart);
        block!(serial.write(addr))
    }
}
