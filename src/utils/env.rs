use std::env;

pub struct Env {
    pub rust_log: String,
}

pub fn get_env() -> Env {
    let rust_log = env::var("RUST_LOG").expect("RUST_LOG must be set");
    Env { rust_log }
}
