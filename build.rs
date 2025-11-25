use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/command/");
    println!("cargo:rerun-if-changed=src/bin/");
    
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    println!("cargo:warning=Building profile: {}", profile);
    println!("cargo:warning=Binaries will be in target/{}/", profile);
}
