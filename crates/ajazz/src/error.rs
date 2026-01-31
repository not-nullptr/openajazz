use hidapi::HidError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetectError {
    #[error(transparent)]
    Hid(#[from] HidError),

    #[error("keyboard not found, please make sure it's plugged in")]
    KeyboardNotFound,
}
