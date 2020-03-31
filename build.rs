extern crate cc;

fn main() {
    // To regenerate bindings, run
    // bindgen --no-layout-tests external/corelocation.h > src/bindings.rs

    cc::Build::new()
        .file("external/corelocation.m")
        .flag("-ObjC")
        .flag("-fmodules")
        .flag("-Fcocoa")
        .flag("-FCorelocation")
        .opt_level(3)
        .compile("libcorelocation.a");
}
