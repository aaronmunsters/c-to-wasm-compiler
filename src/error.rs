use thiserror::Error;

#[derive(Debug, Error)]
/// Error kinds of what can go wrong when a compilation is invoked
pub enum Error {
    #[error("IO Error: {0}")]
    IO(std::io::Error),
    #[error("No Success: {0:?}")]
    Unsuccesful(std::process::Output),
}
