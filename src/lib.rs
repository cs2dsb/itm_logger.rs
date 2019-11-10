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
};

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

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)*) => ();
    ($($arg:tt)*) => ()
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

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => ();
    ($($arg:tt)*) => ()
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

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)*) => ();
    ($($arg:tt)*) => ()
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

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)*) => ();
    ($($arg:tt)*) => ()
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

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)*) => ();
    ($($arg:tt)*) => ()
}