#![feature(rustc_private)]

use rust_verify::config::Args;
use rust_verify::file_loader::PervasiveFileLoader;
use rust_verify::verifier::Verifier;

extern crate rustc_driver;

// For diagnostics when something goes wrong, try "cargo build -vv"

// the build script’s current directory is the source directory of the build script’s package

// path to vstd.rs relative to our directory (source/vstd)
const VSTD_RS_PATH: &str = "../pervasive/vstd.rs";
// path to pervasive relative to our directory (source/vstd)
const PERVASIVE_PATH: &str = "../pervasive";
// name of generated veruslib.vir in install/bin
const VSTD_VIR: &str = "vstd.vir";

fn main() {
    // Consider using links for the rlib paths instead
    // https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/cargo.20build.2Ers.20artifact/near/340569806
    let out_dir = std::env::var("OUT_DIR").unwrap();

    #[cfg(target_os = "macos")]
    let (pre, dl) = ("lib", "dylib");

    #[cfg(target_os = "linux")]
    let (pre, dl) = ("lib", "so");

    #[cfg(target_os = "windows")]
    let (pre, dl) = ("", "dll");

    let TARGET_PATH = std::path::Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap()
        .to_str().unwrap().to_string() + "/";

    let rustc_args: Vec<String> = vec![
        format!("{TARGET_PATH}rust_verify"),
        "--extern".to_string(),
        format!("builtin={TARGET_PATH}libbuiltin.rlib"),
        "--extern".to_string(),
        format!("builtin_macros={TARGET_PATH}{pre}builtin_macros.{dl}"),
        "--extern".to_string(),
        format!("state_machines_macros={TARGET_PATH}{pre}state_machines_macros.{dl}"),
        "--edition=2018".to_string(),
        // "--sysroot".to_string(),
        // todo!(),
        "--cfg".to_string(),
        "erasure_macro_todo".to_string(),
        "--cfg".to_string(),
        "vstd_build_todo".to_string(),
        "--crate-type=lib".to_string(),
        "-Zunpretty=expanded".to_string(),
        "--out-dir".to_string(),
        TARGET_PATH.to_string(),
        VSTD_RS_PATH.to_string(),
    ];

    let mut our_args: Args = Default::default();
    our_args.pervasive_path = Some(PERVASIVE_PATH.to_string());
    our_args.verify_pervasive = true;
    our_args.multiple_errors = 2;
    our_args.export = Some(TARGET_PATH.to_string() + VSTD_VIR);
    our_args.compile = true;
    let file_loader = PervasiveFileLoader::new(Some(PERVASIVE_PATH.to_string()));
    let verifier = Verifier::new(our_args);
    let (_verifier, _stats, status) = rust_verify::driver::run(verifier, rustc_args, file_loader);
    status.expect("failed to build vstd library");

    println!("cargo:rerun-if-changed={PERVASIVE_PATH}");
}
