macro_rules! get_attr {
    ($data:ident; $attr:literal as $pat:pat => $e:expr $(;)?) => {
        match $data.get(&yaml_rust::Yaml::String($attr.to_owned())) {
            Some(s) => match s {
                $pat => Ok(Some($e)),
                _ => Err(crate::error::Error::InvalidAttrType {
                    attr: $attr.to_owned(),
                    pat: stringify!($pat).to_owned(),
                    actual: s.clone(),
                }),
            },
            None => Ok(None),
        }
    };
}
pub(crate) use get_attr;

macro_rules! get_required_attr {
    ($data:ident; $attr:literal as $pat:pat => $e:expr $(;)?) => {
        match $data.get(&yaml_rust::Yaml::String($attr.to_owned())) {
            Some(s) => match s {
                $pat => Ok($e),
                _ => Err(crate::error::Error::InvalidAttrType {
                    attr: $attr.to_owned(),
                    pat: stringify!($pat).to_owned(),
                    actual: s.clone(),
                }),
            },
            None => Err(crate::error::Error::RequiredAttrNotFound($attr.to_owned())),
        }
    };
}
pub(crate) use get_required_attr;

macro_rules! assert_pattern {
    ($data:ident; $pat:pat => $e:expr; attr: $attr:expr $(;)?) => {
        match $data {
            $pat => Ok($e),
            _ => Err(crate::error::Error::InvalidAttrType {
                attr: $attr.to_owned(),
                pat: stringify!($pat).to_owned(),
                actual: $data.clone(),
            }),
        }
    };
}
pub(crate) use assert_pattern;

/// Converts a snake case string to an upper camel case string.
pub fn sc_to_ucc<S: AsRef<str>>(string: S) -> String {
    let mut result = String::new();

    // Prevents an out of bounds error.
    if string.as_ref().is_empty() {
        return result;
    }

    for w in string.as_ref().split('_') {
        result.push_str(&w[0..1].to_uppercase());
        result.push_str(&w[1..w.len()]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sc_to_ucc_test() {
        let input = vec![
            "example_id",
            "oneword",
            "num_at_end1",
            "num_at_end_2",
            "a",
            "",
        ];

        let output: Vec<String> = input.into_iter().map(sc_to_ucc).collect();

        assert_eq!(
            output,
            vec!["ExampleId", "Oneword", "NumAtEnd1", "NumAtEnd2", "A", ""]
        );
    }
}
