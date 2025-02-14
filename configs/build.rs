#![allow(non_snake_case)]


use std::{env, fs};
use serde_derive::{Serialize, Deserialize};
use handlebars::Handlebars;

#[derive(Serialize)]
struct LangTemplateData<'a> {
    rows: Vec<Lang>,
    langs: Vec<&'a str>,
}

#[derive(Serialize, Deserialize)]
struct Lang {
    ident: String,
    zh_CN: String,
}

fn main() {
    let content = fs::read_to_string(env::current_dir().unwrap().join("json").join("lang.json")).unwrap();
    let template = fs::read_to_string(env::current_dir().unwrap().join("tpl").join("lang.rs.tpl")).unwrap();
    let json: Vec<Lang> = serde_json::from_str(&content).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("", template).unwrap();
    let data = LangTemplateData {
        rows: json,
        langs: vec!["zh_CN", "en_US"],
    };
    let rendered = handlebars.render("", &data).unwrap();
    fs::write(&env::current_dir().unwrap().join("src").join("lang.rs"), rendered).unwrap(); // 写入脚本内容到文件系统。
}