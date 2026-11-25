fn main() {
    println!("cargo:rerun-if-changed=src/fast_copy.c");
    let windows = std::env::var_os("CARGO_CFG_WINDOWS").is_some();
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc");
    if windows && msvc {
        cc::Build::new()
            .file("src/fast_copy.c")
            .compile("victory_fast_copy");
    }
}
