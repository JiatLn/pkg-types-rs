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
    with_pkg_json_create_and_remove(|| {
        let pkg_json = PackageJson::read_package_json().unwrap();
        let name = pkg_json.name.unwrap();
        assert_eq!(name, "pkg-name");
    })
}

fn with_pkg_json_create_and_remove<F>(func: F)
where
    F: Fn() -> (),
{
    fs::write("package.json", r#"{"name": "pkg-name"}"#).unwrap();
    func();
    fs::remove_file("package.json").unwrap();
}
