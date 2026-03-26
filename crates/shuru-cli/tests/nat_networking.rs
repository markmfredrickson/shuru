//! Integration tests for NAT networking support.
//!
//! These tests boot a real VM and require built assets (kernel, rootfs,
//! initramfs) and a codesigned binary. They are #[ignore]d by default.
//!
//! Run with:
//!   just build
//!   SHURU_BIN=target/release/shuru cargo test -p shuru-cli -- --ignored

use std::process::Command;

fn shuru_bin() -> String {
    std::env::var("SHURU_BIN").expect(
        "SHURU_BIN not set — point it at a codesigned shuru binary (e.g. just build)",
    )
}

#[test]
#[ignore]
fn nat_can_resolve_dns() {
    let output = Command::new(shuru_bin())
        .args(["run", "--net", "nat", "--", "bash", "-c",
            "curl -so /dev/null -w '%{http_code}' --max-time 10 https://example.com"])
        .output()
        .expect("failed to spawn shuru");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("200"),
        "NAT should allow DNS resolution and HTTP access, got: {}",
        stdout
    );
}

#[test]
#[ignore]
fn proxy_still_works() {
    let output = Command::new(shuru_bin())
        .args(["run", "--net", "proxy", "--", "bash", "-c",
            "curl -so /dev/null -w '%{http_code}' --max-time 10 https://example.com"])
        .output()
        .expect("failed to spawn shuru");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("200"),
        "Proxy networking should still work, got: {}",
        stdout
    );
}

#[test]
#[ignore]
fn no_net_blocks_traffic() {
    let output = Command::new(shuru_bin())
        .args(["run", "--", "bash", "-c",
            "curl -so /dev/null -w '%{http_code}' --max-time 5 https://example.com 2>&1 || echo 'BLOCKED'"])
        .output()
        .expect("failed to spawn shuru");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("200"),
        "No-net mode should block traffic, got: {}",
        stdout
    );
}

#[test]
#[ignore]
fn cli_rejects_invalid_net_mode() {
    let output = Command::new(shuru_bin())
        .args(["run", "--net", "invalid", "--", "echo", "hi"])
        .output()
        .expect("failed to spawn shuru");

    assert!(
        !output.status.success(),
        "Invalid --net mode should fail"
    );
}
