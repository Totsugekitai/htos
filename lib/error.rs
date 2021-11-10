pub struct Error {
    pub kind: ErrorKind,
}

impl Default for Error {
    fn default() -> Self {
        Error {
            kind: ErrorKind::NotSupported,
        }
    }
}

#[non_exhaustive]
pub enum ErrorKind {
    NotFound,
    Empty,
    PermissionDenied,
    InvalidParameter,
    NotSupported,
}
