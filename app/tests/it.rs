use std::path::Path;

use expect_test::expect_file;

#[test]
fn sanity() {
    assert_eq!(2, 1 + 1);
}

#[test]
fn expect_file_test() {
    expect_file!["./snap.txt"].assert_eq("ax12");
}

#[test]
fn expect_file_test_abs() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    expect_file![Path::new(manifest_dir).join("tests/snap.txt")].assert_eq("ax12");
}
