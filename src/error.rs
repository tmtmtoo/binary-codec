pub enum CodecError<UserDefined> {
    Io(std::io::Error),
    UserDefined(UserDefined),
}

impl<UserDefined> std::fmt::Debug for CodecError<UserDefined>
where
    UserDefined: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = match self {
            CodecError::Io(e) => e as &dyn std::fmt::Debug,
            CodecError::UserDefined(e) => e as &dyn std::fmt::Debug,
        };
        debug.fmt(f)
    }
}

impl<UserDefined> std::fmt::Display for CodecError<UserDefined>
where
    UserDefined: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            CodecError::Io(e) => e as &dyn std::fmt::Display,
            CodecError::UserDefined(e) => e as &dyn std::fmt::Display,
        };
        display.fmt(f)
    }
}

impl<UserDefined> std::error::Error for CodecError<UserDefined>
where
    UserDefined: std::error::Error,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let error = match self {
            CodecError::Io(e) => e as &dyn std::error::Error,
            CodecError::UserDefined(e) => e as &dyn std::error::Error,
        };
        error.source()
    }
}
