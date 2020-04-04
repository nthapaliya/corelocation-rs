extern crate corelocation_rs;

use corelocation_rs::Location;

fn main() {
    let location = Location::from_os();

    if let Err(e) = location {
        println!("{}", e);
        std::process::exit(1);
    }

    let location = location.unwrap();

    println!(
        r#"{{
  "latitude": {},
  "longitude": {},
  "altitude": {},
  "vertical_accuracy": {},
  "horizontal_accuracy": {}
}}"#,
        location.latitude,
        location.longitude,
        location.altitude,
        location.v_accuracy,
        location.h_accuracy
    )
}