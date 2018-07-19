extern crate assert_cli;

use assert_cli::Assert;
use std::fs::remove_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn teardown(dirname: &str) {
    if let Err(e) = remove_dir_all(dirname) {
        panic!("Error: {}", e.to_string());
    }
}

#[test]
fn basic() {
    Assert::main_binary()
        .with_args(&["fixtures/basic"])
        .unwrap();
    let txt_path = Path::new("_site/test.txt");
    assert!(txt_path.is_file());
    teardown("_site");
}

#[test]
fn markdown() {
    Assert::main_binary()
        .with_args(&["fixtures/markdown"])
        .unwrap();
    let html_path = Path::new("fixtures/out/markdown/cool.html");
    assert!(html_path.is_file());
    teardown("fixtures/out/markdown");
}

#[test]
fn layouts() {
    Assert::main_binary()
        .with_args(&["fixtures/layouts"])
        .unwrap();
    let layouts_path = Path::new("fixtures/out/layouts/_layouts/hello.hbs");
    assert!(!layouts_path.is_file());

    let mut html_file = File::open("fixtures/out/layouts/test.html").unwrap();
    let mut html_contents = String::new();
    html_file.read_to_string(&mut html_contents).unwrap();
    assert_eq!(html_contents, "test\n\ncool! hello");
    teardown("fixtures/out/layouts");
}

#[test]
fn ignore() {
    Assert::main_binary()
        .with_args(&["fixtures/ignore"])
        .unwrap();

    let html_path = Path::new("fixtures/out/ignore/cool.html");
    assert!(!html_path.is_file());
    teardown("fixtures/out/ignore");
}
