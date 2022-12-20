use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, not_line_ending},
    combinator::opt,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::parser::{without_chars, without_chars_and_line_ending};

use super::{Attribute, AttributeOrNote, Conf, Section};

pub fn custom_conf<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> IResult<&'b str, Conf> + 'c {
    move |input: &'b str| {
        let (input, mut sections) = many0(custom_section(note_starting))(input)?;
        Ok((input, {
            let mut map = BTreeMap::new();
            sections.drain(..).for_each(|(name, section)| {
                map.insert(name, section);
            });
            map
        }))
    }
}

pub fn conf(input: &str) -> IResult<&str, Conf> {
    custom_conf("#")(input)
}

pub fn custom_section<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> IResult<&'b str, Section> + 'c {
    move |input: &'b str| {
        let (input, name) = preceded(
            multispace0,
            terminated(
                delimited(
                    tag("["),
                    without_chars_and_line_ending(("]".to_owned() + note_starting).as_str()),
                    tag("]"),
                ),
                opt(pair(tag(note_starting), not_line_ending)),
            ),
        )(input)?;
        let (input, mut attributes) = preceded(
            multispace0,
            separated_list0(many1(line_ending), custom_attribute_or_note(note_starting)),
        )(input)?;
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
}

pub fn section(input: &str) -> IResult<&str, Section> {
    custom_section(";")(input)
}

pub fn attribute(input: &str) -> IResult<&str, AttributeOrNote> {
    let (input, (s1, s2)) = separated_pair(
        without_chars("[]:"),
        tag(":"),
        without_chars_and_line_ending("[]"),
    )(input)?;
    Ok((
        input,
        AttributeOrNote::Attribute((s1.to_owned(), s2.to_owned())),
    ))
}

pub fn custom_attribute_with_note<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> IResult<&'b str, AttributeOrNote> + 'c {
    move |input: &'b str| {
        let (input, ((k, v), note)) = separated_pair(
            separated_pair(
                without_chars_and_line_ending(("[]:".to_owned() + note_starting).as_str()),
                tag("="),
                without_chars_and_line_ending(("[]".to_owned() + note_starting).as_str()),
            ),
            tag(note_starting),
            not_line_ending,
        )(input)?;
        Ok((
            input,
            AttributeOrNote::AttributeWithNote {
                attribute: (k.to_owned(), v.to_owned()),
                note: note.to_owned(),
            },
        ))
    }
}

pub fn attribute_with_note(input: &str) -> IResult<&str, AttributeOrNote> {
    custom_attribute_with_note("#")(input)
}

pub fn custom_note<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> IResult<&'b str, AttributeOrNote> + 'c {
    move |input: &'b str| {
        let (input, s) =
            preceded(multispace0, preceded(tag(note_starting), not_line_ending))(input)?;
        Ok((input, AttributeOrNote::Note(s.to_owned())))
    }
}

pub fn note(input: &str) -> IResult<&str, AttributeOrNote> {
    custom_note("#")(input)
}

pub fn custom_attribute_or_note<'a: 'b, 'b: 'c, 'c>(
    note_starting: &'a str,
) -> impl FnMut(&'b str) -> IResult<&str, Option<Attribute>> + 'c {
    move |input: &'b str| {
        let (input, may_be_attribute) = alt((
            custom_note(note_starting),
            custom_attribute_with_note(note_starting),
            attribute,
        ))(input)?;
        Ok((
            input,
            match may_be_attribute {
                AttributeOrNote::AttributeWithNote { attribute, .. } => Some(attribute),
                AttributeOrNote::Attribute(attribute) => Some(attribute),
                AttributeOrNote::Note(_) => None,
            },
        ))
    }
}

pub fn attribute_or_note(input: &str) -> IResult<&str, Option<Attribute>> {
    custom_attribute_or_note("#")(input)
}
