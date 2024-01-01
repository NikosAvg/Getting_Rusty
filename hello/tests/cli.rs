// Example test file
// The #[test] tells Rust to run this function when testing
use assert_cmd::Command;
#[test]
fn work() {
    assert!(true);
}
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
