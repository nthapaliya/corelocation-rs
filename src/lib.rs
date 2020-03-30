#[macro_use]
extern crate objc;

use objc::runtime::Object;

#[link(name = "corelocation")]
extern "C" {
    fn run() -> *mut Object;
}

pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub h_accuracy: i64,
    pub altitude: i64,
    pub v_accuracy: i64,
}

impl Location {
    pub fn get_from_os() -> Option<Self> {
        let location: Self;
        let array_len: i64;

        unsafe {
            let obj: *mut Object = run();
            array_len = msg_send![obj, count];
            location = Self {
                latitude: trunc_float(get_float_at_index(obj, 0)),
                longitude: trunc_float(get_float_at_index(obj, 1)),
                altitude: get_int_at_index(obj, 2),
                h_accuracy: get_int_at_index(obj, 3),
                v_accuracy: get_int_at_index(obj, 4),
            }
        }

        if array_len == 0 {
            return None;
        }

        Some(location)
    }
}

fn trunc_float(num: f64) -> f64 {
    let k = 10_000_000.0;
    (num * k).trunc() / k
}

unsafe fn get_float_at_index(nsarray: *mut Object, index: i64) -> f64 {
    let x: *mut Object = msg_send![nsarray, objectAtIndex: index];
    msg_send![x, doubleValue]
}

unsafe fn get_int_at_index(nsarray: *mut Object, index: i64) -> i64 {
    let x: *mut Object = msg_send![nsarray, objectAtIndex: index];
    msg_send![x, integerValue]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_location() {
        // TODO
    }
}
