use pkg_types::PackageJson;

#[test]
fn test_read_pkg_json() {
    let pkg_json = PackageJson::from_path("tests/fixture/package.json");

    match pkg_json {
        Ok(pkg) => {
            dbg!(&pkg);
            let name = pkg.name.unwrap();
            assert_eq!(name, "pkg-name");
        }
        Err(err) => {
            panic!("err: {}", err);
        }
    }
}
