mod decode;
mod encode;

pub use decode::*;
pub use encode::*;

pub enum CodecError<E> {
    Io(std::io::Error),
    UserDefined(E),
}

impl<E> std::fmt::Debug for CodecError<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = match self {
            CodecError::Io(e) => e as &dyn std::fmt::Debug,
            CodecError::UserDefined(e) => e as &dyn std::fmt::Debug,
        };
        debug.fmt(f)
    }
}

impl<E> std::fmt::Display for CodecError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            CodecError::Io(e) => e as &dyn std::fmt::Display,
            CodecError::UserDefined(e) => e as &dyn std::fmt::Display,
        };
        display.fmt(f)
    }
}

impl<E> std::error::Error for CodecError<E>
where
    E: std::error::Error,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let error = match self {
            CodecError::Io(e) => e as &dyn std::error::Error,
            CodecError::UserDefined(e) => e as &dyn std::error::Error,
        };
        error.source()
    }
}
