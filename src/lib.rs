pub mod ini;
pub(crate) mod parser;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn ini() {
        use ini::{attribute, from_str, section, AttributeOrNote};
        assert_eq!(
            attribute("abc:efg\n"),
            Ok(("\n", AttributeOrNote::Attribute(("abc".to_owned(), "efg".to_owned()))))
        );
        assert_eq!(
            section("[section]\nabc:efg\nert:234;efg\n;abc\n"),
            Ok(("\n", ("section".to_owned(), BTreeMap::from([("abc".to_owned(), "efg".to_owned()), ("ert".to_owned(), "234".to_owned())]))))
        );
        let ini = from_str("[abc]\nefg:123\nert:234\n").unwrap();
        assert_eq!(ini["abc"]["efg"], "123");
        assert_eq!(ini["abc"]["ert"], "234");
    }
}
