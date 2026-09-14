use std::env;
use std::path::PathBuf;
use std::process::Command;

fn get_tree_state() -> &'static str {
    let is_clean = Command::new("git")
        .args(["diff", "--stat"])
        .output()
        .unwrap()
        .stdout
        .is_empty();
    if is_clean { "" } else { "dirty-" }
}

fn get_hash() -> String {
    // Github Actions
    if let Ok(hash) = env::var("GITHUB_SHA") {
        return hash[..6].to_string();
    }

    // Everything else
    let output = Command::new("git")
        .args(["rev-parse", "--short=6", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn main() {
    let devkitarm = env::var("DEVKITARM").expect("DEVKITARM must be set");

    let gcc = PathBuf::from(devkitarm)
        .join("bin")
        .join("arm-none-eabi-gcc");

    cc::Build::new()
        .file("asm/3gx_crt0.s")
        .file("asm/csvc.s")
        .compiler(gcc)
        .flags(&[
            "-march=armv6k",
            "-mtune=mpcore",
            "-mfloat-abi=hard",
            "-mtp=soft",
        ])
        .compile("plugin_asm");

    println!("cargo:rerun-if-changed=asm/3gx_crt0.s");
    println!("cargo:rerun-if-changed=asm/csvc.s");
    println!(
        "cargo:rustc-env=GIT_HASH={}{}",
        get_tree_state(),
        get_hash()
    );
}
