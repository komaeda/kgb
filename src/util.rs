use nya::SimpleFile;
use std::path::PathBuf;
use term;

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

pub fn can_be_deleted(path: &PathBuf) -> bool {
    let mut result = path.extension().unwrap() == "hbs";

    let has_path_segment = path.iter().any(|s| {
        let st = s.to_str().unwrap();
        st == "_layouts" || st == "_locales" || st == "_config.toml"
    });

    if has_path_segment {
        result = true;
    }
    result
}

pub fn is_iso6391_code(s: &str) -> bool {
    let codes = vec![
        "ab", "aa", "af", "ak", "sq", "am", "ar", "an", "hy", "as", "av", "ae", "ay", "az", "bm",
        "ba", "eu", "be", "bn", "bh", "bi", "bs", "br", "bg", "my", "ca", "ch", "ce", "ny", "zh",
        "cv", "kw", "co", "cr", "hr", "cs", "da", "dv", "nl", "dz", "en", "eo", "et", "ee", "fo",
        "fj", "fl", "fi", "fr", "ff", "gl", "ka", "de", "el", "gn", "gu", "ht", "ha", "he", "hz",
        "hi", "ho", "hu", "ia", "id", "ie", "ga", "ig", "ik", "io", "is", "it", "iu", "ja", "jv",
        "kl", "kn", "kr", "ks", "kk", "km", "ki", "rw", "ky", "kv", "kg", "ko", "ku", "kj", "la",
        "lb", "lg", "li", "ln", "lo", "lt", "lu", "lv", "gv", "mk", "mg", "ms", "ml", "mt", "mi",
        "mr", "mh", "mn", "na", "nv", "nd", "ne", "ng", "nb", "nn", "no", "ii", "nr", "oc", "oj",
        "cu", "om", "or", "os", "pa", "pi", "fa", "pl", "ps", "pt", "qu", "rm", "rn", "ro", "ru",
        "sa", "sc", "sd", "se", "sm", "sg", "sr", "gd", "sn", "si", "sk", "sl", "so", "st", "es",
        "su", "sw", "ss", "sv", "ta", "te", "tg", "th", "ti", "bo", "tk", "tl", "tn", "to", "tr",
        "ts", "tt", "tw", "ty", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa", "cy", "wo", "fy",
        "xh", "yi", "yo", "za", "zu",
    ];
    codes.iter().any(|c| c == &s)
}

pub fn log(prefix: &str, msg: &str) {
    let mut t = term::stdout().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    t.fg(term::color::MAGENTA).unwrap();
    write!(t, "{} ", prefix).unwrap();
    t.reset().unwrap();
    writeln!(t, "{}", msg).unwrap();
}

#[test]
fn ext_matches_test() {
    use std::collections::HashMap;
    use std::ffi::OsString;

    let mut file = SimpleFile {
        name: OsString::from("coolfile.txt"),
        content: "hello".to_string(),
        rel_path: PathBuf::from(r"coolfile.txt"),
        metadata: HashMap::new(),
    };

    assert!(ext_matches(&mut file, "txt"));
}

#[test]
fn path_includes_test() {
    assert!(path_includes(&PathBuf::from(r"/my/cool/path"), "cool"));
}
