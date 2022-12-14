use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0},
    combinator::{map_res, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

pub type Attribute = (String, String);
pub type Section = (String, BTreeMap<String, String>);
pub type Ini = BTreeMap<String, BTreeMap<String, String>>;

pub fn from_str(str: &str) -> Result<Ini, nom::Err<nom::error::Error<&str>>> {
    Ok(ini(str)?.1)
}

fn ini(input: &str) -> IResult<&str, Ini> {
    let (input, mut sections) = many0(section)(input)?;
    Ok((input, {
        let mut map = BTreeMap::new();
        sections.drain(..).for_each(|(name, section)| {
            map.insert(name, section);
        });
        map
    }))
}

fn section(input: &str) -> IResult<&str, Section> {
    let (input, name) = preceded(opt(multispace0), delimited(tag("["), anystr, tag("]")))(input)?;
    let (input, mut attributes) = preceded(
        opt(multispace0),
        separated_list0(line_ending, opt(attribute)),
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

fn attribute(input: &str) -> IResult<&str, Attribute> {
    Ok(map_res(
        terminated(
            separated_pair(anystr, tag(":"), anystr),
            opt(pair(tag(";"), anystr)),
        ),
        |(s1, s2)| -> Result<(String, String), nom::Err<nom::error::Error<&str>>> {
            Ok((s1.to_owned(), s2.to_owned()))
        },
    )(input)?)
}

fn anystr(input: &str) -> IResult<&str, &str> {
    Ok(("", input))
}
