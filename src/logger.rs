use cortex_m::{
    iprintln,
    peripheral::{
        ITM,
        TPIU,
    },
    interrupt,
};
use log::{
    Log,
    Level,
    Metadata,
    Record,
    SetLoggerError
};

const STIM_PORT_NUMBER: usize = 0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    ImpossibleBaudRate,
}

///Updates the tpiu prescaler to output the desired baud rate
///trace_clk_freq: The frequency of TRACECLKIN in HZ, this is HCLK on most STM32 devices
///                but is implementation specific. Check the ref manual for TRACECLKIN
///baud: The baud rate to set on SWO
///Returns an error if baud > trace_clk_freq or if trace_clk_freq % baud != 0
pub fn update_tpiu_baudrate(trace_clk_freq: u32, baud: u32) -> Result<(), Error> {
    if baud > trace_clk_freq || trace_clk_freq % baud != 0 {
        Err(Error::ImpossibleBaudRate)
    } else {
        let prescaler = (trace_clk_freq / baud) - 1;
        unsafe {
            (*TPIU::PTR).acpr.write(prescaler);
        }
        Ok(())
    }
}

struct ItmLogger {
    enabled: bool,
    log_level: Level,
}

impl Log for ItmLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if !(self.enabled && metadata.level() <= self.log_level) {
            return false;
        }

        #[cfg(feature = "perform-enabled-checks")] 
        unsafe {
            use cortex_m::peripheral::DCB;

            const ITM_TCR_ENABLE_POS: u32 = 0;
            const ITM_TCR_ENABLE_MASK: u32 = 1 << ITM_TCR_ENABLE_POS;

            let itm = &(*ITM::ptr());

            // Check if DEBUGEN is set
            if !DCB::is_debugger_attached() {
                return false;
            }            

            // Check if tracing is enabled
            if itm.tcr.read() & ITM_TCR_ENABLE_MASK == 0 {
                return false;
            }

            // Check if the stim port we're using is enabled
            if itm.ter[0].read() & (1 << (STIM_PORT_NUMBER as u32)) == 0 {
                return false;
            }
        }

        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            unsafe {
                let itm = &mut (*ITM::PTR);
                interrupt::free(|_| {
                    iprintln!(
                        &mut itm.stim[STIM_PORT_NUMBER],
                        "{:<5} [{}] {}",
                        record.level(),
                        record.target(),
                        record.args());
                });
            }
        }
    }

    fn flush(&self) {}
}

static mut LOGGER: ItmLogger = ItmLogger {
    enabled: true,
    log_level: Level::Trace,
};

/// Initialise the logger and set the log level to the provided `log_level`
pub fn init_with_level(log_level: Level) -> Result<(), SetLoggerError> {
    interrupt::free(|_| unsafe {
        log::set_logger(&LOGGER)
    })?;
    log::set_max_level(log_level.to_level_filter());
    Ok(())
}

/// Initialize the logger with default log level (Trace)
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}

/// Wrapper around `init` that panics if an error occurs
pub fn logger_init() {
    init().unwrap();
}

/// Globally disable all logging
pub fn disable_logger() {
    interrupt::free(|_| unsafe {
        LOGGER.enabled = false;
    });
}

/// Globally enable logging, level filtering is still performed
pub fn enable_logger() {
    interrupt::free(|_| unsafe {
        LOGGER.enabled = true;
    });
}
