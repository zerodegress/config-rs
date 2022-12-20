use std::collections::BTreeMap;

pub mod parser;

use self::parser::{conf, custom_conf};

pub type Attribute = (String, String);
pub type Section = (String, BTreeMap<String, String>);
pub type Conf = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, PartialEq, Eq)]
pub enum AttributeOrNote {
    Attribute((String, String)),
    AttributeWithNote {
        attribute: (String, String),
        note: String,
    },
    Note(String),
}

pub fn from_str(str: &str) -> Result<Conf, nom::Err<nom::error::Error<&str>>> {
    Ok(conf(str)?.1)
}

pub fn from_str_custom<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> Result<Conf, nom::Err<nom::error::Error<&'b str>>> + 'c {
    move |str: &'b str| Ok(custom_conf(note_starting)(str)?.1)
}
