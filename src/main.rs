#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate clap;

extern crate comrak;
extern crate config;
extern crate handlebars;
extern crate nya;
extern crate serde;
extern crate term;
extern crate toml;
extern crate yaml_rust;

mod cleanup;
mod filei18n;
mod frontmatter;
mod hbars;
mod layouts;
mod markdown;
mod util;

use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
use std::time::SystemTime;
use util::log;

fn main() {
    let matches = App::new("kgb")
        .version(crate_version!())
        .author("Olivia Hugger <olivia@fastmail.com>")
        .about("A static site generator")
        .subcommand(
            SubCommand::with_name("build").about("Builds a site").arg(
                Arg::with_name("SOURCE")
                    .help("The source directory to generate a site from")
                    .required(false)
                    .index(1),
            ),
        )
        .get_matches();
    
    log("kgb", &format!("Version {}", crate_version!()));
    if let Some(matches) = matches.subcommand_matches("build") {
        log("kgb", "Starting build...");
        let now = SystemTime::now();
        let cdir = std::env::current_dir().unwrap();
        let mut source;

        if matches.is_present("SOURCE") {
            source = matches.value_of("SOURCE").unwrap();
        } else {
            source = cdir.to_str().unwrap();
        }

        let mut confpath = PathBuf::from(source);
        confpath.push("_config.toml");
        let mut config = config::Config::default();
        config
            .merge(config::File::with_name(confpath.to_str().unwrap()))
            .unwrap();

        let default_dest = PathBuf::from("_site");
        let destination = config.get::<PathBuf>("destination").unwrap_or(default_dest);

        let mut ignores: Vec<String> = vec![String::from(".git/*")];
        let mut config_ignores: Vec<String> = config.get("ignore").unwrap_or(Vec::new());
        ignores.append(&mut config_ignores);
        nya::run(
            vec![
                nya::ignore(ignores),
                frontmatter::middleware(),
                layouts::middleware(),
                filei18n::middleware(),
                markdown::middleware(config.clone()),
                hbars::middleware(config.clone()),
                cleanup::middleware(),
            ],
            Some(source),
            Some(destination.to_str().unwrap()),
        ).unwrap();

        log("kgb", &format!("Build finished in {}s", now.elapsed().unwrap().as_secs()));
    }
}
