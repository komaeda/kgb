extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

pub fn lexer(text: &str) -> (&str, &str) {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let marker_end = slice_after_marker.find("---\n").unwrap();
            let yaml_slice = &text[4..marker_end+4];
            let content_slice = &text[marker_end+2*4..];
            (yaml_slice.trim(), content_slice.trim())
        },
        false => panic!("aaaaaaa")
    }
}

pub fn parser(matter: &str) -> Vec<Yaml> {
    YamlLoader::load_from_str(matter).unwrap()
}

#[test]
fn lexer_test() {
    let text = "---\nfoo: bar\n---\n\nContent";
    let (matter, content) = lexer(text);
    assert_eq!(matter, "foo: bar");
    assert_eq!(content, "Content");
}

#[test]
fn parser_test() {
    let text = "---\nfoo: bar\n---\n\nContent";
    let (matter, _) = lexer(text);
    let yaml = parser(matter);
    assert_eq!(yaml[0]["foo"].as_str().unwrap(), "bar");
}
