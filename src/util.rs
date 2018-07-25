use nya::SimpleFile;
use std::path::PathBuf;

pub fn ext_matches(file: &mut SimpleFile, ext: &str) -> bool {
    file.rel_path.to_str().unwrap().ends_with(ext)
}

pub fn rename_ext(file: &mut SimpleFile, ext: &str) {
    let mut pb = PathBuf::from(&file.name.clone().into_string().unwrap());
    pb.set_extension("html");
    file.name = pb.into_os_string();
    file.rel_path.set_extension(ext);
}

pub fn path_includes(path: &PathBuf, segment: &str) -> bool {
    path.iter().any(|s| s.to_str().unwrap() == segment)
}
