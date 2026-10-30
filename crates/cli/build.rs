use std::path::Path;
use std::process::Command;

fn main() {
    // For `rstest` to pick up new tests.
    println!("cargo::rerun-if-changed=tests/reference");

    // The commit this CLI is built from, when it is built from a git checkout
    // (`cargo install --git`, or a clone), states itself in `--version`: a
    // caller holding the commit it expects can tell this build from another
    // of the same version.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let git = |args: &[&str]| -> Option<String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&manifest_dir)
            .args(args)
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let text = String::from_utf8(out.stdout).ok()?;
        let text = text.trim();
        (!text.is_empty()).then(|| text.to_string())
    };
    let described = match git(&["rev-parse", "HEAD"]) {
        Some(commit) => {
            for path in ["HEAD", "packed-refs"] {
                if let Some(p) = git(&["rev-parse", "--git-path", path]) {
                    let p = Path::new(&manifest_dir).join(p);
                    println!("cargo::rerun-if-changed={}", p.display());
                }
            }
            if let Some(head_ref) = git(&["symbolic-ref", "-q", "HEAD"]) {
                if let Some(p) = git(&["rev-parse", "--git-path", &head_ref]) {
                    let p = Path::new(&manifest_dir).join(p);
                    println!("cargo::rerun-if-changed={}", p.display());
                }
            }
            format!("{version} (git {commit})")
        }
        None => version,
    };
    println!("cargo::rustc-env=WASM_BINDGEN_CLI_VERSION={described}");
}
