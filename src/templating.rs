use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct TemplateSchema {
    pub url: String,
    pub shortened: String,
    pub domain: String,
    pub count: String
}

pub fn read_and_apply_templates(path: PathBuf, schema: TemplateSchema) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // Hardcoded templates, will change this if/when the amount of templates increases
    contents
        .replace("{{url}}", &schema.url)
        .replace("{{shortened}}", &schema.shortened)
        .replace("{{domain}}", &schema.domain)
        .replace("{{count}}", &schema.count)
}