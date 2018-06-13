extern crate comrak;

use nya::{create_middleware, SimpleFile};
use util;

pub fn middleware() -> Box<FnMut(&mut Vec<SimpleFile>)> {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        for file in files {
            if util::ext_matches(file, ".md") {
                file.content = comrak::markdown_to_html(
                    file.content.as_str(),
                    &comrak::ComrakOptions::default(),
                );
                util::rename_ext(file, "html");
            }
        }
    })
}
