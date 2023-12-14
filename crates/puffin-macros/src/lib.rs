use std::sync::Mutex;

use once_cell::sync::Lazy;
use rustc_hash::FxHashSet;

// macro hygiene: The user might not have direct dependencies on those crates
#[doc(hidden)]
pub use colored;
#[doc(hidden)]
pub use tracing;

pub static WARNINGS: Lazy<Mutex<FxHashSet<String>>> = Lazy::new(Mutex::default);

/// Warn a user once, with uniqueness determined by the content of the message.
#[macro_export]
macro_rules! warn_once {
    ($($arg:tt)*) => {
        use $crate::colored::Colorize;
        use $crate::tracing::warn;

        if let Ok(mut states) = $crate::WARNINGS.lock() {
            let message = format!("{}", format_args!($($arg)*));
            let formatted = message.bold();
            if states.insert(message) {
                warn!("{formatted}");
            }
        }
    };
}