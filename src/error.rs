use thiserror::Error;

/// All errors used in this crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid E.164 formatted phone number")]
    InvalidPhoneNumber,
}
