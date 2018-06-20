use comrak::{markdown_to_html, ComrakOptions};
use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use util;

pub fn middleware() -> MiddlewareFunction {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        for file in files {
            if util::ext_matches(file, ".md") {
                file.content = markdown_to_html(file.content.as_str(), &ComrakOptions::default());
                util::rename_ext(file, "html");
            }
        }
    })
}
