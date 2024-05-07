use std::{fmt::Display, fs};

use quick_xml::{de::from_str, DeError};
use serde::{de::value::Error, Deserialize, Deserializer};

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
struct People {
    #[serde(rename = "person")]
    people: Vec<Person>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
struct Person {
    #[serde(rename = "@gender")]
    gender: Option<String>,
    name: String,
    age: u32,
    #[serde(deserialize_with = "deserialize_emails")]
    email: String,
    address: Address,
    #[serde(default = "default_other")]
    other: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
struct Address {
    street: String,
    city: String,
    zip: String,
}
fn default_other() -> String {
    String::from("default other value")
}

pub fn deserialize_emails<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    println!("deserializing emails");

    let s = String::deserialize(deserializer)?;

    println!("{s}");
    match s.contains("@") {
        true => Result::Ok(s),
        false => Result::Err(Error::custom("Not a valid email")),
    }
}

pub fn run() -> Result<(), quick_xml::DeError> {
    println!("\n\nDeserializer");
    let xml = fs::read_to_string("./test.xml").unwrap();

    // println!("{}", xml);

    let config: Result<People, DeError> = from_str(&xml);
    dbg!("{?:}", &config);

    Ok(())
}

#[derive(Debug)]
enum CustomError {
    Message(String),
}

impl serde::de::Error for CustomError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        CustomError::Message(msg.to_string())
    }
}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Message(msg) => f.write_str(msg),
        }
    }
}
impl std::error::Error for CustomError {}
