fn main() {
    let env_var = "LINK_TERMBOX_OP";
    let shared_link_type = std::env::var("CARGO_FEATURE_TERMBOX_SHARED_LINK").is_ok();
    let link_path = std::env::var(env_var);
    if let Ok(value) = link_path {
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
