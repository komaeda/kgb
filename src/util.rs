use nya::SimpleFile;

pub fn ext_matches(file: &mut SimpleFile, ext: &str) -> bool {
    file.rel_path.to_str().unwrap().ends_with(ext)
}

pub fn rename_ext(file: &mut SimpleFile, ext: &str) {
    file.abs_path.set_extension(ext);
    file.rel_path.set_extension(ext);
}
