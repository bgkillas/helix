use std::env::var;
fn main() {
    if var("CARGO_CFG_TARGET_OS").unwrap() == "windows"
        && var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32"
    {
        println!(
            "cargo:rustc-link-search=native={}",
            env!("CARGO_MANIFEST_DIR")
        );
        println!("cargo:rustc-link-lib=dylib=lua51");
    }
}
