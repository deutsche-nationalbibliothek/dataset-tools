mod error;

pub use error::{CommandResult, DatasetError, DatasetResult};

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::DatasetError::Other(format!($($arg)*)));
    };
}
