use config::Config;
use comrak::{markdown_to_html, ComrakOptions};
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use util;

pub fn middleware(config: Config) -> MiddlewareFunction {
    create_middleware(move |files: &mut Vec<SimpleFile>| {
        let hardbreaks = config.get::<bool>("markdown.hardbreaks").unwrap_or(false);
        let safe = config.get::<bool>("markdown.safe").unwrap_or(false);
        let smart = config.get::<bool>("markdown.smart").unwrap_or(false);
        let options = ComrakOptions {hardbreaks, safe, smart, ..ComrakOptions::default()};

        for file in files {
            if util::ext_matches(file, ".md") {
                file.content = markdown_to_html(file.content.as_str(), &options);
                util::rename_ext(file, "html");
            }
        }
    })
}
