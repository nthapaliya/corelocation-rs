mod bindings;
mod error;

use bindings::LocInfo;
pub use error::Error;

#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub h_accuracy: i64,
    pub altitude: i64,
    pub v_accuracy: i64,
}

impl LocInfo {
    fn new() -> Self {
        unsafe { bindings::run() }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Location {
    pub fn from_os() -> Result<Self> {
        let l = LocInfo::new();

        match l.status {
            bindings::STATUS_OK => Ok(Self::from_c_struct(l)),
            bindings::STATUS_NOT_ENABLED => Err(Error::NotEnabled),
            bindings::STATUS_NOT_RETURNED => Err(Error::NotReturned),
            bindings::STATUS_STALE => {
                if l.error_duration > 100 {
                    Err(Error::Stale(l.error_duration))
                } else {
                    Ok(Self::from_c_struct(l))
                }
            }
            _ => Err(Error::Unknown),
        }
    }

    fn from_c_struct(l: LocInfo) -> Self {
        Self {
            latitude: l.latitude,
            longitude: l.longitude,
            altitude: l.altitude as i64,
            h_accuracy: l.h_accuracy as i64,
            v_accuracy: l.v_accuracy as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        let location = Location::from_os();
        println!("{:#?}", location);
        panic!();
    }
}
