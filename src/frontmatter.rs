extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
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

pub fn serialize(matter: &Vec<Yaml>) -> String {
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&matter[0]).unwrap();
    }
    out_str
}

pub fn deserialize(matter: &String) -> Vec<Yaml> {
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
fn serializer_test() {
    let text = "---\nfoo: bar\n---\n\nContent";
    let (matter, _) = lexer(text.to_string()).unwrap();
    let dese = deserialize(&matter);
    println!("{:?}", dese);
    let se = serialize(&dese);
    assert_eq!(dese[0]["foo"].as_str().unwrap(), "bar");
    assert_eq!(se, "---\nfoo: bar");
}
