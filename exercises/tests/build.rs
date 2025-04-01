//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

//! This is the build script for both tests7 and tests8.

//! Build script for both tests7 and tests8

fn main() {
    // For tests7 - set TEST_FOO environment variable
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // For tests8 - enable "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
    
    // Re-run if build script changes
    println!("cargo:rerun-if-changed=build.rs");
}