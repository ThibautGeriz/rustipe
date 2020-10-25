use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User must logged")]
    MustBeLogged,
    #[error("Bad credentials")]
    BadCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("unknown error")]
    Unknown,
}
