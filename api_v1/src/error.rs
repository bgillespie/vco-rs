use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SerdeError {
    #[error("Error deserializing DateTime field: \"{0}\"")]
    DateTimeDe(String),

    #[error("Error serializing DateTime field: \"{0}\"")]
    DateTimeSer(String),
}
