// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取环境变量 TEST_FOO 的值
    let test_foo = env::var("TEST_FOO").expect("TEST_FOO environment variable not found");

    // 获取当前时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 将环境变量的值解析为 u64
    let test_foo_value: u64 = test_foo.parse().expect("TEST_FOO is not a number");

    // 检查 TEST_FOO 的值是否在当前时间戳的前后10秒内
    if timestamp >= test_foo_value && timestamp < test_foo_value + 10 {
        // 如果是，输出正确的格式
        println!("cargo:rustc-env=TEST_FOO={}", test_foo_value);
    } else {
        // 如果不是，输出错误信息并退出
        println!("cargo:warning=TEST_FOO value is not within the expected range");
        std::process::exit(1);
    }
}