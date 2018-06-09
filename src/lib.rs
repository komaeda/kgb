extern crate nya;
extern crate comrak;

use nya::{SimpleFile, create_middleware};

fn ext_matches(file: &mut SimpleFile, ext: &str) -> bool {
    file.rel_path.to_str().unwrap().ends_with(ext)
}

fn rename_ext(file: &mut SimpleFile, ext: &str) {
    file.abs_path.set_extension(ext);
    file.rel_path.set_extension(ext);
}

fn markdown_middleware() -> Box<FnMut(&mut Vec<SimpleFile>)> {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        for file in files {
            if ext_matches(file, ".md") {
                file.content = comrak::markdown_to_html(file.content.as_str(), &comrak::ComrakOptions::default());
                rename_ext(file, "html");
            }
        }
    })
}

fn run(source: &str, destination: &str) {
    nya::run(vec![markdown_middleware()], Some(source), Some(destination)).unwrap();
}

#[test]
fn test() {
    run("src/example", "_site");
    assert!(true);
}
