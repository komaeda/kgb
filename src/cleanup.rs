use nya::{create_middleware, SimpleFile, MiddlewareFunction};
use std::path::PathBuf;

pub fn middleware() -> MiddlewareFunction {
  create_middleware(|files: &mut Vec<SimpleFile>| {
    let mut items_to_remove: Vec<usize> = Vec::new();
    {
      let filter = files.iter().filter(|e| path_includes(&e.rel_path, "_layouts"));
      for file in filter {
        let index = files.iter().position(|e| e == file).unwrap();
        items_to_remove.push(index);
      }
    }

    for i in items_to_remove {
      files.remove(i);
    }    
  })
}

fn path_includes(path: &PathBuf, segment: &str) -> bool {
    path.iter().any(|s| s.to_str().unwrap() == segment)
}