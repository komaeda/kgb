use nya::{create_middleware, MiddlewareFunction, SimpleFile};
use util::can_be_deleted;
use std::path::PathBuf;

pub fn middleware() -> MiddlewareFunction {
    create_middleware(|files: &mut Vec<SimpleFile>| {
        let mut items_to_remove: Vec<PathBuf> = Vec::new();
        {
            let filter = files
                .iter()
                .filter(|e| can_be_deleted(&e.rel_path));
            for file in filter {
                items_to_remove.push(file.rel_path.clone());
            }
        }

        for i in items_to_remove {
            println!("{}", i.to_str().unwrap());
            let index = files.iter().position(|e| e.rel_path == i).unwrap();
            files.remove(index);
        }
    })
}
