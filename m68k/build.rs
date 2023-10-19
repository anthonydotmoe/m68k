use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host_triple = env::var("HOST").unwrap();

    if host_triple == target {
        println!("cargo:rustc-cfg=native");
    }
}