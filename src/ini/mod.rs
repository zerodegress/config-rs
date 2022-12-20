use std::collections::BTreeMap;

use self::parser::{custom_ini, ini};

pub mod parser;

pub type Attribute = (String, String);
pub type Section = (String, BTreeMap<String, String>);
pub type Ini = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, PartialEq, Eq)]
pub enum AttributeOrNote {
    Attribute((String, String)),
    AttributeWithNote {
        attribute: (String, String),
        note: String,
    },
    Note(String),
}

pub fn from_str(str: &str) -> Result<Ini, nom::Err<nom::error::Error<&str>>> {
    Ok(ini(str)?.1)
}

pub fn from_str_custom<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> Result<Ini, nom::Err<nom::error::Error<&'b str>>> + 'c {
    move |str: &'b str| Ok(custom_ini(note_starting)(str)?.1)
}
