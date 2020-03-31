use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    NotEnabled,
    NotReturned,
    Stale(i32),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnabled => write!(f, "Location services not enabled"),
            Self::NotReturned => write!(f, "Unable to get location result"),
            Self::Stale(d) => write!(f, "Location result too stale: {} seconds old", d),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}

// TODO: Figure out what this actually does
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
