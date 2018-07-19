#[macro_use]
extern crate serde_json;

extern crate clap;
extern crate comrak;
extern crate config;
extern crate handlebars;
extern crate nya;
extern crate serde;
extern crate yaml_rust;

mod cleanup;
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

    let mut confpath = PathBuf::from(source);
    confpath.push("_config.toml");
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name(confpath.to_str().unwrap()))
        .unwrap();

    let default_dest = PathBuf::from("_site");
    let destination = config.get::<PathBuf>("destination").unwrap_or(default_dest);

    let default_ignore: Vec<String> = Vec::new();
    let ignores = config.get("ignore").unwrap_or(default_ignore);
    println!("hello!");
    println!("{:#?}", ignores);
    nya::run(
        vec![
            nya::ignore(ignores),
            frontmatter::middleware(),
            layouts::middleware(),
            markdown::middleware(),
            cleanup::middleware(),
        ],
        Some(source),
        Some(destination.to_str().unwrap()),
    ).unwrap();
}
