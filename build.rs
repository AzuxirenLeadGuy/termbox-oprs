fn main() {
    let shared_link_type = std::env::var("CARGO_FEATURE_TERMBOX_SHARED_LINK").is_ok();
    if let Ok(value) = std::env::var("LINK_TERMBOX_OP") {
        println!("cargo::rustc-link-search=native={}", value);
    }
    let libname = "termboxop";
    let prefix = "cargo::rustc-link-lib";
    if shared_link_type {
        println!("{}=dylib={}", prefix, libname);
    } else {
        println!("{}=static={}", prefix, libname);
    }
}
