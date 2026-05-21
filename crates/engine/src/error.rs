use std::fmt;

#[derive(Debug)]
pub enum EngineError {

    Io(std::io::Error),

    Protocol(String),

    ProcessExited,

    InvalidState(String),
}

impl fmt::Display for EngineError {

    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            EngineError::Io(e) => {
                write!(f, "io error: {}", e)
            }

            EngineError::Protocol(e) => {
                write!(f, "protocol error: {}", e)
            }

            EngineError::ProcessExited => {
                write!(f, "engine process exited")
            }

            EngineError::InvalidState(e) => {
                write!(f, "invalid state: {}", e)
            }
        }
    }
}

impl std::error::Error for EngineError {}

impl From<std::io::Error>
for EngineError {

    fn from(
        e: std::io::Error,
    ) -> Self {

        EngineError::Io(e)
    }
}