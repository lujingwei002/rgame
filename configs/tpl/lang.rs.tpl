#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fs;
use std::path::{Path};
use serde_derive::Deserialize;

#[derive(Default)]
pub enum LanguageType {
    #[default]
{{#each langs}}
{{this}},
{{/each}}
}

#[derive(Default, Deserialize)]
pub struct Language {
    ident: String,
    {{#each langs}}
    {{this}}: String,
    {{/each}}
}

#[derive(Default)]
pub struct Lang {
    _lang: LanguageType,
    {{#each rows}}
    _{{ident}}: Language,
    {{/each}}
}

impl Lang {

    pub fn set(&mut self, lang: LanguageType) {
        self._lang = lang;
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Lang {
        let path = path.as_ref().join("lang.json");
        let content = fs::read_to_string(path.as_path()).unwrap();
        let mut l = Lang::default();
        let json: Vec<Language> = serde_json::from_str(&content).unwrap();
        json.iter().for_each(|lang| {
            match lang.ident.as_str() {
                {{#each rows}}
                "{{ident}}"=>{
                    {{#each ../langs}}
                    l._{{../ident}}.{{this}} = lang.{{this}}.clone();
                    {{/each}}
                }
                {{/each}}
                _=>{
                    println!("未定义的变量名 {:?}", lang.ident);
                }
            }
        });
        l
    }

    {{#each rows}}
    /// {{zh_CN}}
    pub fn {{ident}}(&self) -> &str {
        match self._lang {
            {{#each ../langs}}
            LanguageType::{{this}} => &self._{{../ident}}.{{this}}.as_str(),
            {{/each}}
        }
    }

    {{/each}}

}
