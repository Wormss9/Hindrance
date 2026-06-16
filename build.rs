use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = env::var("TARGET")?;
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let icon = manifest_dir.join("assets/icon.ico");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    if target.contains("windows") {
        let rc_path = out_dir.join("icon.rc");
        let res_path = out_dir.join("icon.res");

        std::fs::write(
            &rc_path,
            format!(
                "1 ICON \"{}\"",
                icon.to_string_lossy().replace("\\", "\\\\")
            ),
        )?;

        let status = Command::new("x86_64-w64-mingw32-windres")
            .args([
                "--target=x86_64-w64-mingw32",
                rc_path.to_str().unwrap(),
                "-O",
                "coff",
                "-o",
                res_path.to_str().unwrap(),
            ])
            .status()?;

        if !status.success() {
            panic!("windres failed");
        }

        println!("cargo:rustc-link-arg={}", res_path.display());
    }

    Ok(())
}
