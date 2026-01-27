use assert_cmd::cargo;

#[test]
fn runs() {
    let mut cmd = cargo::cargo_bin_cmd!("hello");
    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert!(
        stdout.contains("Hello") && stdout.contains("Shanghai") && stdout.contains("Nashville")
    );
}

#[test]
fn true_ok() {
    let mut cmd = cargo::cargo_bin_cmd!("true");
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = cargo::cargo_bin_cmd!("false");
    cmd.assert().failure();
}
