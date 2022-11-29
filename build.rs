use std::{process::Command, path::PathBuf};
use std::io::{self, Write};

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR").to_string();
    // Define the flatc path
    let flatc_path = PathBuf::from(manifest_dir.as_str())
        .join("tools")
        .join("flatc.exe");
    // Make sure it exists
    if !flatc_path.exists() {
        panic!(
            "{} doesn't exist!",
            flatc_path.as_os_str().to_str().unwrap()
        );
    };
    let flatc = Command::new(flatc_path)
        .arg("-r")
        .arg("schema.fbs")
        .current_dir(PathBuf::from(manifest_dir.as_str()).join("src"))
        .output()
        .expect("Failed to execute flatc command");
    io::stdout().write_all(&flatc.stdout).unwrap();
    io::stderr().write_all(&flatc.stderr).unwrap();
    assert!(flatc.status.success());
}
