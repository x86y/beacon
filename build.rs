use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cbqn_dir = Path::new(&out_dir).join("CBQN");

    if !cbqn_dir.exists() {
        Command::new("git")
            .args([
                "clone",
                "https://github.com/dzaima/CBQN.git",
                cbqn_dir.to_str().unwrap(),
            ])
            .status()
            .expect("cargo:warning=Failed to clone CBQN repository");
    }

    Command::new("make")
        .current_dir(&cbqn_dir)
        .arg("static-lib")
        .arg("FFI=0")
        .status()
        .expect("cargo:warning=Failed to build CBQN static library");

    println!("cargo:rustc-link-search=native={}", cbqn_dir.display());
    println!("cargo:rustc-link-lib=static=cbqn");

    println!("cargo:rerun-if-changed={}", cbqn_dir.join("src").display());
}
