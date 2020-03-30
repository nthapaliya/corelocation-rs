extern crate cc;

fn main() {
    cc::Build::new()
        .file("external/corelocation.m")
        .flag("-ObjC")
        .flag("-fmodules")
        .flag("-Fcocoa")
        .flag("-FCorelocation")
        .opt_level(3)
        .compile("libcorelocation.a");
}
