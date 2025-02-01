use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let status = Command::new("cargo")
        .args(["bootimage"])
        .status()
        .expect("Failed to run cargo bootimage");

    if !status.success() {
        panic!("bootimage failed!");
    }
}
