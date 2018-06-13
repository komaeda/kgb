extern crate nya;
extern crate comrak;

mod markdown;
mod util;

fn run(source: &str, destination: &str) {
    nya::run(vec![markdown::middleware()], Some(source), Some(destination)).unwrap();
}

#[test]
fn test() {
    run("example", "_site");
    assert!(true);
}
