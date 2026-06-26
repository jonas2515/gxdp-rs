use std::path::PathBuf;
use std::process::Command;

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // prevent linking libraries to avoid documentation failure
        return;
    }

    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        std::process::exit(1);
    }
    
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("meson-build");

    // Configure with meson
    let mut setup = Command::new("meson");
    setup
        .args(["setup"])
        .arg(&build_dir)
        .arg(&manifest_dir);
    if build_dir.exists() {
        setup.arg("--reconfigure");
    }
    let status = setup.status().expect("failed to run meson setup");
    assert!(status.success(), "meson setup failed");

    // Build the library with meson
    let status = Command::new("meson")
        .args(["compile", "-C"])
        .arg(&build_dir)
        .status()
        .expect("failed to run meson compile");
    assert!(status.success(), "meson compile failed");

    // Link the static library
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=gxdp");

    // Rebuild when the submodule sources or build definition change
    println!("cargo:rerun-if-changed=subprojects/libgxdp");
    println!("cargo:rerun-if-changed=meson.build");
}
