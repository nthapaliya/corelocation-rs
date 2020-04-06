mod bindings;
mod error;

use bindings::LocInfo;
pub use error::Error;

/*
 * Reference: https://en.wikipedia.org/wiki/Decimal_degrees
 *
 * decimal places | decimal degrees | N/S or E/W at equator
 * 0              | 1.0             | 111.32 km
 * 1              | 0.1             | 11.132 km
 * 2              | 0.01            | 1.1132 km
 * 3              | 0.001           | 111.32 m
 * 4              | 0.0001          | 11.132 m
 * 5              | 0.00001         | 1.1132 m
 * 6              | 0.000001        | 111.32 mm
 * 7              | 0.0000001       | 11.132 mm
 * 8              | 0.00000001      | 1.1132 mm
 */

// In meters
const ACCURACY: [f64; 9] = [
    111_320.0, 11_132.0, 1_113.0, 111.0, 11.0, 1.0, 0.1, 0.01, 0.001,
];

fn trim_to_precision(n: f64, accuracy: f64) -> f64 {
    for (i, v) in ACCURACY.iter().enumerate() {
        if accuracy > *v {
            let base: f64 = 10.0;
            let k = base.powf(i as f64);
            return (k * n).trunc() / k;
        }
    }

    n
}

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
            latitude: trim_to_precision(l.latitude, l.h_accuracy),
            longitude: trim_to_precision(l.longitude, l.h_accuracy),
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

    #[test]
    fn test_truncate() {
        let longitude: f64 = -74.189_714_162_760;
        let h_accuracy: f64 = 65.0;

        assert_eq!(trim_to_precision(longitude, h_accuracy), -74.1897);
    }
}
