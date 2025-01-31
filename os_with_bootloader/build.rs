use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Use `bootimage` to create a bootable disk image
    let status = Command::new("cargo")
        .args(["bootimage"])
        .status()
        .expect("Failed to run cargo bootimage");

    if !status.success() {
        panic!("bootimage failed!");
    }

    println!("Bootimage created successfully!");
}
