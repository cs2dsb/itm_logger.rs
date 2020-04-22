#![no_std]

//! An implementation of the [log facade](http://crates.io/crates/log) that sends logging information over ITM stim port 0
//!
//! Calls to logging macros optimize down to nothing if `logging` feature is not enabled.
//! 
//! # Usage
//! ```
//! use itm_logger::{
//!     logger_init,
//!     update_tpiu_baudrate,
//!     log,
//!     info,
//!     error,
//!     Level,
//! };
//! 
//! logger_init();
//! // if you change the CPU clock during boot up
//! let sysclk: Hertz = clocks.sysclk().into();
//! update_tpiu_baudrate(sysclk.0, ITM_BAUD_RATE).expect("Failed to reset TPIU baudrate");
//! ```
//!
//! # `perform-enabled-checks` feature
//! Not enabled by default
//!
//! Enabling this feature causes the logger to attempt to check if ITM is enabled before writing to it.
//!
//! This *should* work on all cortex-m except M0. See [is_debugger_attached documentation](https://docs.rs/cortex-m/0.6.2/cortex_m/peripheral/struct.DCB.html#method.is_debugger_attached)
//! for more information.
//!
//! Checks performed are:
//! * Debug is enabled (DCB::DHCSR::DEBUGEN)
//! * ITM is enabled (ITM::TCR::ITMENA)
//! * Stim port we're using is enabled (ITM::TER[PORT])

#[cfg(feature = "logging")]
pub use log::{
    log,
    Level,
};

#[cfg(feature = "logging")]
mod logger;

#[cfg(feature = "logging")]
pub use logger::{
    logger_init,
    init_with_level,
    update_tpiu_baudrate,
    disable_logger,
    enable_logger,
};

/// A macro that accepts logging syntax but does no logging. It `let _ =` each of the parameters
/// to avoid unused expression warnings when logging is disabled.
#[macro_export]
macro_rules! stub {
    (target: $target:expr, $( $arg:expr$(,)?)+ ) => (
        {
            let _ = $target;
            $(let _ = $arg;)+
            ()
        }
    );
    ( $( $arg:expr$(,)?)+ ) => (
        {
            $(let _ = $arg;)+
            ()
        }
    )
}

/// Re-export of `log::error` if logging feature is enabled, `stub` if not
#[cfg(not(feature = "logging"))]
pub use self::stub as error;

/// Re-export of `log::warn` if logging feature is enabled, `stub` if not
#[cfg(not(feature = "logging"))]
pub use self::stub as warn;

/// Re-export of `log::info` if logging feature is enabled, `stub` if not
#[cfg(not(feature = "logging"))]
pub use self::stub as info;

/// Re-export of `log::debug` if logging feature is enabled, `stub` if not
#[cfg(not(feature = "logging"))]
pub use self::stub as debug;

/// Re-export of `log::trace` if logging feature is enabled, `stub` if not
#[cfg(not(feature = "logging"))]
pub use self::stub as trace;

/// Re-export of `log::error` if logging feature is enabled, `stub` if not
#[cfg(feature = "logging")]
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, Level::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(Level::Error, $($arg)*);
    )
}

/// Re-export of `log::warn` if logging feature is enabled, `stub` if not
#[cfg(feature = "logging")]
pub use log::warn as warn;

/// Re-export of `log::info` if logging feature is enabled, `stub` if not
#[cfg(feature = "logging")]
pub use log::info as info;

/// Re-export of `log::debug` if logging feature is enabled, `stub` if not
#[cfg(feature = "logging")]
pub use log::debug as debug;

/// Re-export of `log::trace` if logging feature is enabled, `stub` if not
#[cfg(feature = "logging")]
pub use log::trace as trace;