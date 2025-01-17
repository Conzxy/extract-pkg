use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let exe_name = "RePKG.exe";
    let out_dir = env::var("OUT_DIR").unwrap();
    let src = Path::new("asset").join(exe_name);

    println!("cargo:rerun-if-changed={}", src.display());
    fs::copy(src, Path::new(&out_dir).join("../../../").join(exe_name)).unwrap();
}