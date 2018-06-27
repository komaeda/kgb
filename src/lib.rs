#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate comrak;
extern crate handlebars;
extern crate nya;
extern crate serde;
extern crate toml;
extern crate yaml_rust;

mod config;
mod frontmatter;
mod layouts;
mod markdown;
mod util;

pub fn run(source: &str) {
    let config = config::read_config(source).unwrap();
    let mut hbars = handlebars::Handlebars::new();
    let default_dest = std::path::PathBuf::from("_site");
    let destination = config.destination.as_ref().unwrap_or(&default_dest);
    nya::run(
        vec![
            frontmatter::middleware(),
            layouts::register_mw(&mut hbars, &config),
            markdown::middleware(),
        ],
        Some(source),
        Some(destination.to_str().unwrap()),
    ).unwrap();
}
