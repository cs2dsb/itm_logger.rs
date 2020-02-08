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

const ITM_TCR_ENABLE_POS: u32 = 0;
const ITM_TCR_ENABLE_MASK: u32 = 1 << ITM_TCR_ENABLE_POS;
const STIM_PORT_NUMBER: usize = 0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    ImpossibleBaudRate,
}

pub fn logger_init() {
    init().unwrap();
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
            (*TPIU::ptr()).acpr.write(prescaler);
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
        self.enabled && metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            unsafe {
                let itm = &mut (*ITM::ptr());
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

pub fn init_with_level(log_level: Level) -> Result<(), SetLoggerError> {
    interrupt::free(|_| unsafe {
        log::set_logger(&LOGGER)
    })?;
    log::set_max_level(log_level.to_level_filter());
    Ok(())
}

pub fn disable_logger() {
    interrupt::free(|_| unsafe {
        LOGGER.enabled = false;
    });
}

pub fn enable_logger() {
    interrupt::free(|_| unsafe {
        LOGGER.enabled = true;
    });
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}