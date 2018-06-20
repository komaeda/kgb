#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate comrak;
extern crate nya;
extern crate serde;
extern crate toml;
extern crate yaml_rust;

mod config;
mod frontmatter;
mod markdown;
mod util;

fn run(source: &str) {
    let config = config::read_config(source).unwrap();
    let destination = config.destination.unwrap_or("_site".to_string());
    nya::run(
        vec![frontmatter::middleware(), markdown::middleware()],
        Some(source),
        Some(destination.as_str()),
    ).unwrap();
}

#[test]
fn test() {
    run("example");
    assert!(true);
}
