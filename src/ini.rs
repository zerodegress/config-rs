use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, satisfy, not_line_ending},
    combinator::opt,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, branch::alt,
};

use crate::parser::{anystr, without_chars, without_chars_and_line_ending};

pub type Attribute = (String, String);
pub type Section = (String, BTreeMap<String, String>);
pub type Ini = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AttributeOrNote {
    Attribute((String, String)),
    AttributeWithNote{
        attribute: (String, String),
        note: String
    },
    Note(String)
}

pub fn from_str(str: &str) -> Result<Ini, nom::Err<nom::error::Error<&str>>> {
    Ok(ini(str)?.1)
}

pub(crate) fn ini(input: &str) -> IResult<&str, Ini> {
    let (input, mut sections) = many0(section)(input)?;
    Ok((input, {
        let mut map = BTreeMap::new();
        sections.drain(..).for_each(|(name, section)| {
            map.insert(name, section);
        });
        map
    }))
}

pub(crate) fn section(input: &str) -> IResult<&str, Section> {
    let (input, name) = preceded(multispace0, delimited(tag("["), without_chars("]"), tag("]")))(input)?;
    let (input, mut attributes) =
        preceded(multispace0, separated_list0(many1(line_ending), attribute_or_note))(input)?;
    Ok((input, {
        let mut map = BTreeMap::new();
        attributes
            .drain(..)
            .filter_map(|x| if let Some(x) = x { Some(x) } else { None })
            .for_each(|(k, v)| {
                map.insert(k, v);
            });
        (name.to_owned(), map)
    }))
}

pub(crate) fn attribute(input: &str) -> IResult<&str, AttributeOrNote> {
    let (input, (s1, s2)) = separated_pair(delimited(multispace0, without_chars_and_line_ending(":;"), multispace0), tag(":"), not_line_ending)(input)?;
    Ok((input, AttributeOrNote::Attribute((s1.to_owned(), s2.to_owned()))))
}

pub(crate) fn attribute_with_note(input: &str) -> IResult<&str, AttributeOrNote> {
    let (input, ((k, v), note)) = separated_pair(separated_pair(delimited(multispace0, without_chars_and_line_ending(":;"), multispace0), tag(":"), without_chars_and_line_ending(";")), tag(";"), not_line_ending)(input)?;
    Ok((input, AttributeOrNote::AttributeWithNote { attribute: (k.to_owned(), v.to_owned()), note: note.to_owned() }))
}

pub(crate) fn note(input: &str) -> IResult<&str, AttributeOrNote> {
    let (input, s) = preceded(multispace0, preceded(tag(";"), not_line_ending))(input)?;
    Ok((input, AttributeOrNote::Note(s.to_owned())))
}

pub(crate) fn attribute_or_note(input: &str) -> IResult<&str, Option<Attribute>> {
    let (input, may_be_attribute) = alt((note, attribute_with_note, attribute))(input)?;
    Ok((input, match may_be_attribute {
        AttributeOrNote::AttributeWithNote { attribute, .. } => Some(attribute),
        AttributeOrNote::Attribute(attribute) => Some(attribute),
        AttributeOrNote::Note(_) => None,
    }))
}
