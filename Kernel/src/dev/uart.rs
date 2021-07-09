/*! Universal asynchronous receiver-transmitter */

use core::fmt;

use crate::arch::dev::uart::HwUart;

/**
 * Simple arch independent interface for UART writing
 */
pub struct Uart {
    m_hw_uart: HwUart
}

impl Uart {
    /**
     * Constructs an uninitialized `Uart` which must be initialized with
     * `Uart::init()`
     */
    pub const fn new() -> Self {
        Self { m_hw_uart: HwUart::CONST_NEW }
    }

    /**
     * Initializes the underling hardware to make it active and ready to
     * receive bytes to write
     */
    pub fn init(&mut self) -> bool {
        self.m_hw_uart.init_hw()
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_hw_uart.write_str(s)
    }
}

/**
 * Common interface used by the `UartWriterInner` to use the hardware
 * implementations
 */
pub trait HwUartBase: fmt::Write {
    /**
     * Constructs an uninitialized `HwUartBase` based object
     */
    const CONST_NEW: Self;

    /**
     * Initializes the underling hardware to make it active and ready to
     * receive bytes to write.
     *
     * The method is ensured by the upper encapsulating obj that is
     * called only once per instance
     */
    fn init_hw(&mut self) -> bool;
}