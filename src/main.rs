#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate clap;
extern crate comrak;
extern crate handlebars;
extern crate nya;
extern crate serde;
extern crate toml;
extern crate yaml_rust;

mod cleanup;
mod config;
mod frontmatter;
mod layouts;
mod markdown;
mod util;

use clap::{App, Arg};
use std::path::PathBuf;

fn main() {
    let matches = App::new("kg")
        .version("0.0.4")
        .author("Olivia Hugger <olivia@fastmail.com>")
        .about("A static site generator")
        .arg(
            Arg::with_name("SOURCE")
                .help("The source directory to generate a site from")
                .required(true)
                .index(1),
        )
        .get_matches();

    let source = matches.value_of("SOURCE").unwrap();
    let config = config::read_config(source).unwrap();
    let default_dest = PathBuf::from("_site");
    let destination = config.destination.as_ref().unwrap_or(&default_dest);
    nya::run(
        vec![
            frontmatter::middleware(),
            layouts::middleware(&config),
            markdown::middleware(),
            cleanup::middleware(),
        ],
        Some(source),
        Some(destination.to_str().unwrap()),
    ).unwrap();
}
