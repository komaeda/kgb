use config::Config;
use handlebars::Handlebars;
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use std::path::PathBuf;
use util;

pub fn register_mw(hbars: &'static mut Handlebars, config: &Config) -> MiddlewareFunction {
    //create_middleware(|files: &mut Vec<SimpleFile>| {
    //    let layout_files = files
    //        .into_iter()
    //        .filter(|e| path_includes(&e.rel_path, "_layouts"));
    //    for file in layout_files {
    //        hbars.register_template_string(file.rel_path.file_stem().unwrap().to_str().unwrap(), file.content);
    //    }
    //})
    create_middleware(|_: &mut Vec<SimpleFile>| {
        println!("{:?}", hbars);
    })
}

fn path_includes(path: &PathBuf, segment: &str) -> bool {
    path.iter().any(|s| s.to_str().unwrap() == segment)
}
