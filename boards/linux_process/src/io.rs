use core::fmt::Write;
use core::panic::PanicInfo;

use posix;

use kernel::debug;
use kernel::debug::IoWrite;

use crate::CHIP;
use crate::PROCESSES;

// /// Writer is used by kernel::debug to panic message to the serial port.
pub struct Writer;

// /// Global static for debug writer
pub static mut WRITER: Writer = Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

impl IoWrite for Writer {
    fn write(&mut self, buf: &[u8]) {
        let uart = unsafe { &mut linux::console::CONSOLE };

        for &c in buf {
            uart.send_byte(c);
        }
    }
}

struct NoLed;

impl kernel::hil::led::Led for NoLed {
    fn init(&mut self) {}
    fn on(&mut self) {}
    fn off(&mut self) {}
    fn toggle(&mut self) {}
    fn read(&self) -> bool {
        false
    }
}

/// Panic handler.
pub unsafe fn panic_fmt(info: &PanicInfo) -> ! {
    let mut led = NoLed;
    let writer = &mut WRITER;
    debug::panic(
        &mut [&mut led],
        writer,
        info,
        &posix::support::nop,
        &PROCESSES,
        &CHIP,
    )
}
