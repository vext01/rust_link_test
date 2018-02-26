use std::process::Command;
use std::path::Path;
use std::env;

const C_DEPS_PATH: &'static str = "c_lib";

fn build_libcstuff() {
    env::set_current_dir(&Path::new(C_DEPS_PATH)).unwrap();
    let res = Command::new("make")
        .output()
        .unwrap();
    assert!(res.status.success());
    env::set_current_dir(&Path::new("..")).unwrap();
}

fn main() {
    build_libcstuff();

    println!("cargo:rustc-link-lib=cstuff");
    println!("cargo:rustc-link-search={}", C_DEPS_PATH);
    println!("cargo:rerun-if-changed={}", C_DEPS_PATH);
}
