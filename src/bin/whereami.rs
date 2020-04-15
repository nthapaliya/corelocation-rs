extern crate corelocation_rs;

use corelocation_rs::{Location, Locator};

fn main() {
    let location = Location::get();

    if let Err(e) = location {
        println!(
            r#"{{
  "error": "{}"
}}"#,
            e
        );
        std::process::exit(1);
    }

    let location = location.unwrap();

    println!(
        r#"{{
  "latitude": {},
  "longitude": {},
  "altitude": {},
  "v_accuracy": {},
  "h_accuracy": {}
}}"#,
        location.latitude,
        location.longitude,
        location.altitude,
        location.v_accuracy,
        location.h_accuracy
    )
}
