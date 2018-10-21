use frontmatter::deserialize;
use handlebars::Handlebars;
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use util::path_includes;
use yaml_rust::Yaml;

pub fn middleware() -> MiddlewareFunction {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        let mut hbars = Handlebars::new();
        {
            let layout_files_no_fm = files.iter().filter(|e| filter_helper(e, false));

            for file in layout_files_no_fm {
                let template_name = &file.rel_path.file_stem().unwrap().to_str().unwrap();
                hbars
                    .register_template_string(template_name, &file.content)
                    .unwrap();
            }
        }

        for file in files {
            let fm = file.metadata.get("frontmatter");
            if let Some(frontmatter) = fm {
                let de = deserialize(frontmatter);
                if let Some(e) = de[0].as_hash().unwrap().get(&Yaml::from_str("layout")) {
                    file.content = hbars
                        .render(e.as_str().unwrap(), &json!({"content": file.content}))
                        .unwrap();
                }
            }
        }
    })
}

fn filter_helper(e: &SimpleFile, with_fm: bool) -> bool {
    let mut has_layout = false;
    let fm = e.metadata.get("frontmatter");
    if let Some(frontmatter) = fm {
        let de = deserialize(frontmatter);
        if de[0].as_hash().unwrap().get(&Yaml::from_str("layout")).is_some() {
            has_layout = true;
        }
    }
    if with_fm {
        has_layout = !has_layout;
    }
    path_includes(&e.rel_path, "_layouts") && !has_layout
}
