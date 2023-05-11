use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "commonjs")]
    Commonjs,
}

impl Default for TypeEnum {
    fn default() -> Self {
        TypeEnum::Commonjs
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BugsField {
    String(String),
    UrlAndEmail { url: String, email: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryField {
    String(String),
    UrlAndType {
        type_: String,
        url: String,
        directory: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinField {
    String(String),
    Record(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManField {
    String(String),
    StringVec(Vec<String>),
}

/// A “person” is an object with a “name” field and optionally “url” and “email”. Or you can shorten that all into a single string, and npm will parse it for you.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageJsonPerson {
    String(String),
    Object {
        name: String,
        email: Option<String>,
        url: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportsField {
    String(String),
    Record(HashMap<RecordKey, RecordValue>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RecordKey {
    Import,
    Require,
    Dot,
    Node,
    Browser,
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordValue {
    String(String),
    HashMap(HashMap<RecordValueHashMapKey, String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RecordValueHashMapKey {
    Import,
    Require,
    String(String),
}
