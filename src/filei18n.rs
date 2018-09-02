use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use std::ffi::OsString;
use std::path::PathBuf;
use util::is_iso6391_code;

pub fn middleware() -> MiddlewareFunction {
    create_middleware(move |files: &mut Vec<SimpleFile>| {
        for file in files {
            let fileclone = file.name.clone();
            let filename = fileclone.to_str().unwrap();
            let split_filename: Vec<&str> = filename.split(|e| e == '.').collect();
            if split_filename.len() >= 3 {
                let (last, rest) = split_filename.split_last().unwrap();
                let (last2, rest2) = rest.split_last().unwrap();

                if rest2.len() > 0 && is_iso6391_code(last2) && last == &"md" {
                    let new_name = vec![rest2.join(".").as_str(), last].join(".");
                    let mut new_path = PathBuf::from(file.rel_path.to_str().unwrap());
                    new_path.pop();
                    new_path.push(&new_name);
                    new_path = PathBuf::from(last2).join(new_path);
                    file.name = OsString::from(new_name);
                    file.rel_path = new_path;
                }
            }
        }
    })
}
