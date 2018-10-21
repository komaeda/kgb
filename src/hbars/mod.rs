use config::Config;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext};
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use std::ffi::OsString;
use std::path::PathBuf;
use util::{ext_matches, path_includes};

mod locales;

pub fn middleware(config: Config) -> MiddlewareFunction {
    create_middleware(move |files: &mut Vec<SimpleFile>| {
        let mut hbars = Handlebars::new();
        let ctxmap = locales::generate_locale_map(files, &config);

        // This is the Handlebars helper that is used to pull locale-specific keys.
        let t_helper =
            |h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut Output| {
                let key = h.param(0).unwrap();
                let ctx = h.param(1).unwrap();
                let value = ctx
                    .value()
                    .get(key.value().as_str().unwrap())
                    .unwrap()
                    .as_str()
                    .unwrap();
                out.write(value)?;
                Ok(())
            };
        hbars.register_helper("t", Box::new(t_helper));

        let mut filevec: Vec<SimpleFile> = Vec::new();

        for file in &mut files.clone() {
            if (ext_matches(file, "hbs") && !path_includes(&file.rel_path, "_layouts"))
                || ext_matches(file, "html")
            {
                let name = config
                    .get::<String>("name")
                    .unwrap_or_else(|_| "My Site".to_string());

                if ctxmap.len() == 1 {
                    hbars
                        .register_template_string(file.name.to_str().unwrap(), &file.content)
                        .unwrap();
                    let meta = json!({
                        "site": {
                            "name": name,
                        },
                        "l": &ctxmap.values().next().unwrap(),
                    });
                    let mut file_struct = SimpleFile {
                        name: name_to_html(&file.name),
                        content: hbars.render(file.name.to_str().unwrap(), &meta).unwrap(),
                        rel_path: file.rel_path.clone(),
                        metadata: file.metadata.clone(),
                    };
                    file_struct.rel_path.set_extension("html");
                    filevec.push(file_struct);
                } else {
                    for (locale, ctx) in &ctxmap {
                        let templatename = format!("{}_{}", file.name.to_str().unwrap(), &locale);
                        hbars
                            .register_template_string(templatename.as_str(), &file.content)
                            .unwrap();

                        let meta = json!({
                            "site": {
                                "name": name,
                            },
                            "l": &ctx,
                        });
                        let mut file_struct = SimpleFile {
                            name: name_to_html(&file.name),
                            content: hbars.render(templatename.as_str(), &meta).unwrap(),
                            rel_path: locale_rel(&file.rel_path, locale),
                            metadata: file.metadata.clone(),
                        };
                        filevec.push(file_struct);
                    }
                }
            }
        }

        for f in filevec {
            files.push(f);
        }
    })
}

fn locale_rel(path: &PathBuf, locale: &str) -> PathBuf {
    let mut p = PathBuf::from(format!("/{}", locale));
    p.push(path);
    p.set_extension("html");
    p
}

fn name_to_html(name: &OsString) -> OsString {
    let mut pb = PathBuf::from(name.clone().into_string().unwrap());
    pb.set_extension("html");
    pb.into_os_string()
}
