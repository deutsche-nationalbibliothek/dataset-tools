mod error;
mod params;
mod vocab;

pub use error::{CommandResult, DatasetError, DatasetResult};
pub use params::Parameters;

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::DatasetError::Other(format!($($arg)*)))
    };
}
