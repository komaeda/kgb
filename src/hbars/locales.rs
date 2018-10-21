use config::Config;
use nya::SimpleFile;
use std::collections::HashMap;
use std::ffi::OsString;
use toml::Value;

pub fn generate_locale_map(files: &mut Vec<SimpleFile>, config: &Config) -> HashMap<String, Value> {
    let locales = config
        .get::<Vec<String>>("locales")
        .unwrap_or_else(|_| vec![String::from("en")]);
    let mut ctxmap: HashMap<String, Value> = HashMap::new();
    if locales.len() == 1 {
        // This assumes that if you only have one locale, you don't need
        // any specific locale files, and therefore loads an empty default
        // locale.
        let t = "".parse::<Value>().unwrap();
        ctxmap.insert("en".to_string(), t);
    } else {
        for (i, locale) in locales.iter().enumerate() {
            let t;
            let locale_file = files
                .iter()
                .find(|&f| f.name == OsString::from(format!("{}.toml", locale)));
            if let Some(f) = locale_file {
                t = (&f.content).parse::<Value>().unwrap();
            } else {
                t = "".parse::<Value>().unwrap();
            }
            let locale = locales[i].clone();
            ctxmap.insert(locale, t);
        }
    }

    ctxmap
}
