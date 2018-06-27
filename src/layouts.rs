use config::Config;
use handlebars::Handlebars;
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use frontmatter::deserialize;
use yaml_rust::Yaml;
use std::path::PathBuf;

pub fn middleware(_config: &Config) -> MiddlewareFunction {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        let mut hbars = Handlebars::new();
        {
            let layout_files = files
                .iter()
                .filter(|e| path_includes(&e.rel_path, "_layouts"));
            for file in layout_files {
                let template_name = &file.rel_path.file_stem().unwrap().to_str().unwrap();
                hbars.register_template_string(template_name, &file.content).unwrap();
            }
        }

        for file in files {
            let fm = file.metadata.get("frontmatter");
            if let Some(frontmatter) = fm {
                let de = deserialize(frontmatter);
                if let Some(e) = de[0].as_hash().unwrap().get(&Yaml::from_str("layout")) {
                    file.content = hbars.render(e.as_str().unwrap(), &json!({"content": file.content})).unwrap();
                }
            }
        }
    })
}

fn path_includes(path: &PathBuf, segment: &str) -> bool {
    path.iter().any(|s| s.to_str().unwrap() == segment)
}
