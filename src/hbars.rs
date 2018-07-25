use handlebars::Handlebars;
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use util::{ext_matches, rename_ext};
use config::Config;
use std::ffi::OsString;
use fluent::MessageContext;

pub fn middleware(config: Config) -> MiddlewareFunction {
    create_middleware(move |files: &mut Vec<SimpleFile>| {
        let mut hbars = Handlebars::new();
        let locales = config.get::<Vec<String>>("locales").unwrap_or(vec![String::from("en")]);
        for locale in locales {
            let mut ctx = MessageContext::new(&[]);
            {
                let locale_file = files.iter().find(|&f| f.name == OsString::from(format!("{}.ftl", locale)));
                if let Some(f) = locale_file {
                    ctx.add_messages(&f.content);
                }
            }

            for file in files {
                if ext_matches(file, "hbs") || ext_matches(file, "html") {
                    let name = config.get::<String>("name").unwrap_or("My Site".to_string());
                    let meta = json!({
                        "site": {
                            "name": name,
                        },
                    });
                    hbars.register_template_string(file.name.to_str().unwrap(), &file.content).unwrap();
                    file.content = hbars
                        .render(file.name.to_str().unwrap(), &meta)
                        .unwrap();

                    rename_ext(file, "html");
                }
            }
        }
    })
}

