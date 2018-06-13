extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};
use nya::{create_middleware, SimpleFile};

pub fn middleware() -> Box<FnMut(&mut Vec<SimpleFile>)> {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        for file in files {
            let lex = lexer(file.content.clone());
            if let Some((matter, content)) = lex {
                file.metadata.insert("frontmatter", matter.to_string());
                file.content = content.to_string();
            }
        }
    })
}

pub fn lexer(text: String) -> Option<(String, String)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let marker_end = slice_after_marker.find("---\n").unwrap();
            let yaml_slice = &text[4..marker_end+4];
            let content_slice = &text[marker_end+2*4..];
            Some((yaml_slice.trim().to_string(), content_slice.trim().to_string()))
        },
        false => None
    }
}

pub fn parser(matter: String) -> Vec<Yaml> {
    YamlLoader::load_from_str(matter.as_str()).unwrap()
}

#[test]
fn lexer_test() {
    let text = "---\nfoo: bar\n---\n\nContent";
    let (matter, content) = lexer(text.to_string()).unwrap();
    assert_eq!(matter, "foo: bar".to_string());
    assert_eq!(content, "Content".to_string());
}

#[test]
fn parser_test() {
    let text = "---\nfoo: bar\n---\n\nContent";
    let (matter, _) = lexer(text.to_string()).unwrap();
    let yaml = parser(matter);
    assert_eq!(yaml[0]["foo"].as_str().unwrap(), "bar".to_string());
}
