#[macro_use]
extern crate objc;

mod error;

pub use error::Error;
use objc::runtime::{Class, Object};

#[link(name = "corelocation")]
extern "C" {
    #[link_name = "OBJC_CLASS_$_LocationService"]
    static SERVICE: Class;
}

#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub h_accuracy: i64,
    pub altitude: i64,
    pub v_accuracy: i64,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Location {
    unsafe fn from_objc_object(obj: *mut Object) -> Result<Self> {
        match msg_send!(obj, errorCode) {
            0 => (),
            1 => return Err(Error::NotEnabled),
            2 => return Err(Error::NotReturned),
            3 => return Err(Error::Stale(msg_send!(obj, errorDuration))),
            _ => return Err(Error::Unknown),
        }

        Ok(Self {
            latitude: trunc_float(msg_send!(obj, latitude)),
            longitude: trunc_float(msg_send!(obj, longitude)),
            altitude: msg_send!(obj, altitude),
            h_accuracy: msg_send!(obj, horizontalAccuracy),
            v_accuracy: msg_send!(obj, verticalAccuracy),
        })
    }

    pub fn from_os() -> Result<Self> {
        let location: Result<Self>;

        unsafe {
            #[allow(clippy::deref_addrof)]
            let obj: *mut Object = msg_send![&SERVICE, new];

            let _: i64 = msg_send![obj, run];
            location = Self::from_objc_object(obj)
        }

        location
    }
}

fn trunc_float(num: f64) -> f64 {
    let k = 10_000_000.0;
    (num * k).trunc() / k
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
