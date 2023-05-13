use pkg_types::PackageJson;
use std::fs;

#[test]
fn test_read_pkg_json_from_path() {
    let pkg_json = PackageJson::default();
    assert!(pkg_json.write().is_ok());
    fs::remove_file("package.json").unwrap();
}
