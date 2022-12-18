use std::collections::BTreeMap;

#[test]
#[cfg(feature = "ini")]
fn ini() {
    use crate::ini::{attribute, from_str, section, AttributeOrNote};
    assert_eq!(
        attribute("key:value"),
        Ok((
            "",
            AttributeOrNote::Attribute(("key".to_owned(), "value".to_owned()))
        ))
    );
    assert_eq!(
        section("[section];note1\nkey1:value1;note2\nkey2:value2\n"),
        Ok((
            "\n",
            (
                "section".to_owned(),
                BTreeMap::from([
                    ("key1".to_owned(), "value1".to_owned()),
                    ("key2".to_owned(), "value2".to_owned())
                ])
            )
        ))
    );
    assert_eq!(
        from_str("[section1]\nkey1:value1\nkey2:value2\n\n[section2]\n\n\nkey3:value3"),
        Ok(BTreeMap::from([
            (
                "section1".to_owned(),
                BTreeMap::from([
                    ("key1".to_owned(), "value1".to_owned()),
                    ("key2".to_owned(), "value2".to_owned())
                ])
            ),
            (
                "section2".to_owned(),
                BTreeMap::from([("key3".to_owned(), "value3".to_owned())])
            )
        ]))
    );
}
