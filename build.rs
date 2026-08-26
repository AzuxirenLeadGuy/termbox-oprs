fn main() {
    let env_var = "LINK_TERMBOX_OP";
    let link_path = std::env::var(env_var);
    if let Ok(value) = link_path {
        println!("cargo::rustc-link-search=native={}", value);
    }
}
