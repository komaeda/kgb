extern crate kg;

use std::path::Path;
use std::fs::remove_dir_all;

fn teardown(dirname: &str) {
    match remove_dir_all(dirname) {
        Err(e) => {
            panic!("Error: {}", e.to_string());
        },
        _ => (),
    }
}

#[test]
fn basic() {
    kg::run("fixtures/basic");
    let txt_path = Path::new("_site/test.txt");
    assert!(txt_path.is_file());
    teardown("_site");
}
