use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Bad credentials")]
    BadCredentials,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("unknown error")]
    Unknown,
}
