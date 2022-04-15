/// Prints and returns the value of a given expression for quick and dirty debugging.
///
/// Like [`std::dbg`], but used [`crate::Debug`] instead of [`std::fmt::Debug`]
///
/// # Stability
///
/// The exact output printed by this macro should not be relied upon and is subject to future changes.
///
/// # Panics
///
/// Panics if writing to [`std::io::stderr`] fails.
#[macro_export]
macro_rules! dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}] {} = {}",
                    file!(),
                    line!(),
                    stringify!($val),
                    $crate::pprint(&tmp)
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        $crate::dbg!($($val:expr),+)
    };
}
