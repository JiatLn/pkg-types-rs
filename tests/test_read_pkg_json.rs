use pkg_types::PackageJson;
use std::fs;

#[test]
fn test_read_pkg_json_from_path() {
    let pkg_json = PackageJson::from_path("tests/fixture/package.json").unwrap();
    let name = pkg_json.name.unwrap();
    assert_eq!(name, "pkg-name");
}

#[test]
fn test_read_pkg_json_from_current_dir() {
    fs::write("package.json", r#"{"name": "pkg-name"}"#).unwrap();
    let pkg = PackageJson::read_package_json();
    fs::remove_file("package.json").unwrap();
    assert!(pkg.is_ok());
}
