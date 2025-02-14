#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fs;
use std::path::{Path};
use serde_derive::Deserialize;

#[derive(Default)]
pub enum LanguageType {
    #[default]
zh_CN,
en_US,
}

#[derive(Default, Deserialize)]
pub struct Language {
    ident: String,
    zh_CN: String,
    en_US: String,
}

#[derive(Default)]
pub struct Lang {
    _lang: LanguageType,
    _MISSING_OPEN_ID_FIELD: Language,
    _MISSING_PLATFORM_FIELD: Language,
    _MISSING_CHANNEL_FIELD: Language,
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
                "MISSING_OPEN_ID_FIELD"=>{
                    l._MISSING_OPEN_ID_FIELD.zh_CN = lang.zh_CN.clone();
                    l._MISSING_OPEN_ID_FIELD.en_US = lang.en_US.clone();
                }
                "MISSING_PLATFORM_FIELD"=>{
                    l._MISSING_PLATFORM_FIELD.zh_CN = lang.zh_CN.clone();
                    l._MISSING_PLATFORM_FIELD.en_US = lang.en_US.clone();
                }
                "MISSING_CHANNEL_FIELD"=>{
                    l._MISSING_CHANNEL_FIELD.zh_CN = lang.zh_CN.clone();
                    l._MISSING_CHANNEL_FIELD.en_US = lang.en_US.clone();
                }
                _=>{
                    println!("未定义的变量名 {:?}", lang.ident);
                }
            }
        });
        l
    }

    /// 缺少open_id字段
    pub fn MISSING_OPEN_ID_FIELD(&self) -> &str {
        match self._lang {
            LanguageType::zh_CN => &self._MISSING_OPEN_ID_FIELD.zh_CN.as_str(),
            LanguageType::en_US => &self._MISSING_OPEN_ID_FIELD.en_US.as_str(),
        }
    }

    /// 缺少platform字段
    pub fn MISSING_PLATFORM_FIELD(&self) -> &str {
        match self._lang {
            LanguageType::zh_CN => &self._MISSING_PLATFORM_FIELD.zh_CN.as_str(),
            LanguageType::en_US => &self._MISSING_PLATFORM_FIELD.en_US.as_str(),
        }
    }

    /// 缺少channel字段
    pub fn MISSING_CHANNEL_FIELD(&self) -> &str {
        match self._lang {
            LanguageType::zh_CN => &self._MISSING_CHANNEL_FIELD.zh_CN.as_str(),
            LanguageType::en_US => &self._MISSING_CHANNEL_FIELD.en_US.as_str(),
        }
    }


}
