//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
// use::std::env;

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?

        println!("cargo:rustc-env=TEST_FOO={}", timestamp);

        // 确保输出命令也符合 `cargo:KEY=VALUE` 格式
        println!("cargo:command=Your command here with {}, please checkout exercises/tests/build.rs", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    println!("cargo:rustc-cfg=feature=\"pass\"");
    let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("{}", your_command);
}
