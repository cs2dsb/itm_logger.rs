#![no_std]

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
    update_tpiu_baudrate,
    disable_logger,
    enable_logger,
};

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

#[cfg(not(feature = "logging"))]
pub use self::stub as error;

#[cfg(not(feature = "logging"))]
pub use self::stub as warn;

#[cfg(not(feature = "logging"))]
pub use self::stub as info;

#[cfg(not(feature = "logging"))]
pub use self::stub as debug;

#[cfg(not(feature = "logging"))]
pub use self::stub as trace;

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

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => (
        log::log!(target: $target, Level::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(Level::Warn, $($arg)*);
    )
}

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, Level::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(Level::Info, $($arg)*);
    )
}

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, Level::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(Level::Debug, $($arg)*);
    )
}

#[cfg(feature = "logging")]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, Level::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(Level::Trace, $($arg)*);
    )
}