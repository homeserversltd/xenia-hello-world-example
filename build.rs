fn main() {
    const ZERO_SHA: &str = "0000000000000000000000000000000000000000";
    let source_sha = std::env::var("CARTRIDGE_SOURCE_SHA").unwrap_or_else(|_| ZERO_SHA.into());
    let valid = source_sha.len() == 40
        && source_sha
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !valid {
        panic!("CARTRIDGE_SOURCE_SHA must be exactly 40 lowercase hexadecimal characters");
    }
    println!("cargo:rerun-if-env-changed=CARTRIDGE_SOURCE_SHA");
    println!("cargo:rustc-env=CARTRIDGE_SOURCE_SHA={source_sha}");
}
