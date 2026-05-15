#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::core::log(
            $crate::logger::level::INFO,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::core::log(
            $crate::logger::level::WARNING,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::core::log(
            $crate::logger::level::ERROR,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::core::log(
            $crate::logger::level::DEBUG,
            &format!($($arg)*)
        )
    };
}