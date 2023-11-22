use itertools::PeekNth;

pub fn expect<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
    expected_char: char,
) -> Result<(), String> {
    let found_char = input.next();
    found_char.map_or_else(
        || Err(format!("expected {expected_char} but found end of input")),
        |found_byte| {
            if found_byte == expected_char {
                Ok(())
            } else {
                Err(format!("expected {expected_char} but found {found_byte}"))
            }
        },
    )
}

pub fn parse_variable<I: Iterator<Item = char>>(input: &mut PeekNth<I>) -> Result<String, String> {
    let mut inner = || {
        expect(input, '{')?;
        expect(input, '{')?;
        let mut identifier = String::new();
        match input.next() {
            Some('#') => {
                return Err(
                    "expected first character of variable identifier but found # indicating start \
                     of each directive"
                        .to_owned(),
                );
            }
            Some('/') => {
                return Err(
                    "expected first character of variable identifier but found / indicating end \
                     of each directive"
                        .to_owned(),
                );
            }
            Some(byte) => identifier.push(byte),
            None => {
                return Err("expected variable identifier but found end of input".to_owned());
            }
        }
        loop {
            match input.next() {
                Some('}') => break,
                Some(byte) => {
                    identifier.push(byte);
                }
                None => {
                    return Err("expected }} but found end of input".to_owned());
                }
            }
        }
        expect(input, '}')?;
        Ok(identifier)
    };
    inner().map_err(|err| format!("{err}\nwhile parsing variable"))
}

pub fn parse_partial_block_partial<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<(), String> {
    let mut inner = || {
        for character in "{{>@partial-block}}".chars() {
            expect(input, character)?;
        }
        Ok::<_, String>(())
    };
    inner().map_err(|err| format!("{err}\nwhile parsing partial block"))
}

pub fn parse_each<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<(String, Vec<Child>), String> {
    let mut inner = || {
        expect(input, '{')?;
        expect(input, '{')?;
        expect(input, '#')?;
        expect(input, 'e')?;
        expect(input, 'a')?;
        expect(input, 'c')?;
        expect(input, 'h')?;
        expect(input, ' ')?;
        let mut identifier = String::new();
        loop {
            match input.next() {
                Some('}') => break,
                Some(byte) => {
                    identifier.push(byte);
                }
                None => {
                    return Err("expected }} but found end of input".to_owned());
                }
            }
        }
        expect(input, '}')?;
        let children = parse_children(input)?;
        expect(input, '{')?;
        expect(input, '{')?;
        expect(input, '/')?;
        expect(input, 'e')?;
        expect(input, 'a')?;
        expect(input, 'c')?;
        expect(input, 'h')?;
        expect(input, '}')?;
        expect(input, '}')?;
        Ok((identifier, children))
    };
    inner().map_err(|err| format!("{err}\nwhile parsing each"))
}

pub fn parse_partial_block<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<(String, Vec<Child>), String> {
    // https://handlebarsjs.com/guide/partials.html#partial-blocks
    let mut inner = || {
        expect(input, '{')?;
        expect(input, '{')?;
        expect(input, '#')?;
        expect(input, '>')?;
        let mut partial_name = String::new();
        loop {
            match input.next() {
                Some('}') => break,
                Some(byte) => {
                    partial_name.push(byte);
                }
                None => {
                    return Err("expected }} but found end of input".to_owned());
                }
            }
        }
        expect(input, '}')?;
        let children = parse_children(input)?;
        expect(input, '{')?;
        expect(input, '{')?;
        expect(input, '/')?;
        for character in partial_name.chars() {
            expect(input, character)?;
        }
        expect(input, '}')?;
        expect(input, '}')?;
        Ok((partial_name, children))
    };
    inner().map_err(|err| format!("{err}\nwhile parsing partial"))
}

#[derive(PartialEq, Eq, Debug)]
pub enum AttributeValuePart {
    Literal(String),
    Variable(String),
}

pub fn parse_attribute_value<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<Vec<AttributeValuePart>, String> {
    let mut inner = || -> Result<_, String> {
        let mut result = Vec::new();
        loop {
            match input.peek() {
                Some('"') | None => return Ok(result),
                Some('{') => result.push(AttributeValuePart::Variable(parse_variable(input)?)),
                Some(_) => {
                    let other = input.next().unwrap();
                    match result.last_mut() {
                        Some(AttributeValuePart::Literal(value)) => value.push(other),
                        _ => result.push(AttributeValuePart::Literal(other.to_string())),
                    }
                }
            }
        }
    };
    inner().map_err(|err| format!("{err}\nwhile parsing attribute value"))
}

#[derive(PartialEq, Eq, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: Option<Vec<AttributeValuePart>>,
}

pub fn parse_attribute<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<Attribute, String> {
    let mut inner = || -> Result<_, String> {
        let mut key = String::new();
        let has_value = loop {
            match input.peek() {
                Some('=') => {
                    input.next().unwrap();
                    break true;
                }
                Some('>') => {
                    break false;
                }
                Some(byte) if byte.is_ascii_whitespace() => {
                    break false;
                }
                Some(_) => {
                    let byte = input.next().unwrap();
                    key.push(byte);
                }
                None => {
                    return Err("expected = but found end of input".to_owned());
                }
            }
        };
        if has_value {
            expect(input, '"')?;
            let value = parse_attribute_value(input)?;
            expect(input, '"')?;
            Ok(Attribute {
                key,
                value: Some(value),
            })
        } else {
            Ok(Attribute { key, value: None })
        }
    };
    inner().map_err(|err| format!("{err}\nwhile parsing attribute"))
}

pub fn parse_attributes<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<Vec<Attribute>, String> {
    let mut inner = || -> Result<_, String> {
        let mut attributes = Vec::new();
        loop {
            match input.peek() {
                Some('>') | None => return Ok(attributes),
                Some(byte) if byte.is_ascii_whitespace() => {
                    assert!(input.next().unwrap().is_ascii_whitespace(), "unreachable");
                }
                Some(_) => attributes.push(parse_attribute(input)?),
            }
        }
    };
    inner().map_err(|err| format!("{err}\nwhile parsing an arbitrary amount of attributes"))
}

#[derive(PartialEq, Eq, Debug)]
pub enum Child {
    Element(Element),
    Literal(String),
    Variable(String),
    Each(String, Vec<Child>),
    PartialBlock(String, Vec<Child>),
    PartialBlockPartial,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Child>,
}

pub fn parse_children<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<Vec<Child>, String> {
    let mut inner = || -> Result<_, String> {
        let mut result = Vec::new();
        loop {
            match input.peek() {
                None => return Ok(result),
                Some('<') => match input.peek_nth(1) {
                    Some('/') => return Ok(result),
                    Some(_) => result.push(Child::Element(parse_element(input)?)),
                    None => {
                        return Err("expected / as part of closing tag or tag name as part of \
                                    child but found end of input"
                            .to_owned());
                    }
                },
                Some('{') => match input.peek_nth(1) {
                    Some('{') => match input.peek_nth(2) {
                        Some('/') => return Ok(result),
                        Some('>') => match input.peek_nth(3) {
                            Some('@') => {
                                parse_partial_block_partial(input)?;
                                result.push(Child::PartialBlockPartial);
                            }
                            _ => {
                                // TODO FIXME partial without inner elements
                                return Err("TODO".to_owned());
                            }
                        },
                        Some('#') => {
                            match input.peek_nth(3) {
                                Some('>') => {
                                    // partial block (which may contain inner elements)
                                    let (partial_name, children) = parse_partial_block(input)?;
                                    result.push(Child::PartialBlock(partial_name, children));
                                }
                                _ => {
                                    let (identifier, children) = parse_each(input)?;
                                    result.push(Child::Each(identifier, children));
                                }
                            }
                        }
                        Some(_) => result.push(Child::Variable(parse_variable(input)?)),
                        None => {
                            return Err("expected # as part of each directive or start of \
                                        variable identifier but found end of input"
                                .to_owned());
                        }
                    },
                    Some(byte) => {
                        return Err(format!(
                            "expected {{ as part of each directive or variable but found {byte}"
                        ));
                    }
                    None => {
                        return Err(
                            "expected { as part of each directive or variable but found end of \
                             input"
                                .to_owned(),
                        );
                    }
                },
                Some(_) => {
                    let other = input.next().unwrap();
                    match result.last_mut() {
                        Some(Child::Literal(value)) => value.push(other),
                        _ => result.push(Child::Literal(other.to_string())),
                    }
                }
            }
        }
    };
    inner().map_err(|err| format!("{err}\nwhile parsing children"))
}

pub fn parse_element<I: Iterator<Item = char>>(input: &mut PeekNth<I>) -> Result<Element, String> {
    let mut inner = || -> Result<_, String> {
        expect(input, '<')?;
        let mut name = String::new();
        let mut attributes = Vec::new();
        loop {
            match input.next() {
                Some(byte) if byte.is_ascii_whitespace() => {
                    attributes = parse_attributes(input)?;
                    expect(input, '>')?;
                    break;
                }
                Some('>') => {
                    break;
                }
                Some(byte) => {
                    name.push(byte);
                }
                None => {
                    return Err("expected whitespace or > but found end of input".to_owned());
                }
            }
        }
        // https://html.spec.whatwg.org/dev/syntax.html#void-elements
        match name.as_str() {
            "!doctype" | "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input"
            | "link" | "meta" | "source" | "track" | "wbr" => Ok(Element {
                name,
                attributes,
                children: Vec::new(),
            }),
            _ => {
                let children = parse_children(input)?;
                expect(input, '<')?;
                expect(input, '/')?;
                for character in name.chars() {
                    expect(input, character)?;
                }
                expect(input, '>')?;
                Ok(Element {
                    name,
                    attributes,
                    children,
                })
            }
        }
    };
    inner().map_err(|err| format!("{err}\nwhile parsing element"))
}

#[cfg(test)]
mod tests {
    use itertools::peek_nth;

    use crate::html_recursive_descent::{
        parse_attribute, parse_attribute_value, parse_attributes, parse_children, parse_element,
        parse_partial_block, parse_partial_block_partial, parse_variable, Attribute,
        AttributeValuePart, Child, Element,
    };

    #[test]
    fn variable_1() {
        assert_eq!(
            Ok("test".to_owned()),
            parse_variable(&mut peek_nth("{{test}}".chars()))
        );
    }

    #[test]
    fn variable_2() {
        assert_eq!(
            Err("expected } but found end of input\nwhile parsing variable".to_owned()),
            parse_variable(&mut peek_nth("{{}}".chars()))
        );
    }

    #[test]
    fn variable_3() {
        assert_eq!(
            Err("expected } but found end of input\nwhile parsing variable".to_owned()),
            parse_variable(&mut peek_nth("{{test}".chars()))
        );
    }

    #[test]
    fn variable_4() {
        assert_eq!(
            Err("expected }} but found end of input\nwhile parsing variable".to_owned()),
            parse_variable(&mut peek_nth("{{test".chars()))
        );
    }

    #[test]
    fn variable_5() {
        assert_eq!(
            Err(
                "expected variable identifier but found end of input\nwhile parsing variable"
                    .to_owned()
            ),
            parse_variable(&mut peek_nth("{{".chars()))
        );
    }

    #[test]
    fn variable_6() {
        assert_eq!(
            Err("expected { but found end of input\nwhile parsing variable".to_owned()),
            parse_variable(&mut peek_nth("{".chars()))
        );
    }

    #[test]
    fn variable_7() {
        assert_eq!(
            Err("expected { but found end of input\nwhile parsing variable".to_owned()),
            parse_variable(&mut peek_nth("".chars()))
        );
    }

    #[test]
    fn attribute_value_1() {
        assert_eq!(Ok(vec![]), parse_attribute_value(&mut peek_nth("".chars())));
    }

    #[test]
    fn attribute_value_2() {
        assert_eq!(
            Ok(vec![AttributeValuePart::Literal("test".to_owned())]),
            parse_attribute_value(&mut peek_nth("test".chars()))
        );
    }

    #[test]
    fn attribute_value_3() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Variable("a".to_owned()),
                AttributeValuePart::Literal("test".to_owned())
            ]),
            parse_attribute_value(&mut peek_nth("{{a}}test".chars()))
        );
    }

    #[test]
    fn attribute_value_4() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Literal("test".to_owned()),
                AttributeValuePart::Variable("a".to_owned()),
            ]),
            parse_attribute_value(&mut peek_nth("test{{a}}".chars()))
        );
    }

    #[test]
    fn attribute_value_5() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Variable("a".to_owned()),
                AttributeValuePart::Literal("test".to_owned()),
                AttributeValuePart::Variable("b".to_owned()),
            ]),
            parse_attribute_value(&mut peek_nth("{{a}}test{{b}}".chars()))
        );
    }

    #[test]
    fn attribute_value_6() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Literal("a".to_owned()),
                AttributeValuePart::Variable("test".to_owned()),
            ]),
            parse_attribute_value(&mut peek_nth("a{{test}}".chars()))
        );
    }

    #[test]
    fn attribute_value_7() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Variable("test".to_owned()),
                AttributeValuePart::Literal("a".to_owned()),
            ]),
            parse_attribute_value(&mut peek_nth("{{test}}a".chars()))
        );
    }

    #[test]
    fn attribute_value_8() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Literal("a".to_owned()),
                AttributeValuePart::Variable("test".to_owned()),
                AttributeValuePart::Literal("b".to_owned()),
            ]),
            parse_attribute_value(&mut peek_nth("a{{test}}b".chars()))
        );
    }

    #[test]
    fn attribute_value_9() {
        assert_eq!(
            Ok(vec![AttributeValuePart::Variable("test".to_owned()),]),
            parse_attribute_value(&mut peek_nth("{{test}}".chars()))
        );
    }

    // TODO FIXME
    #[test]
    fn attribute_1() {
        assert_eq!(
            Ok(Attribute {
                key: String::new(),
                value: Some(vec![])
            }),
            parse_attribute(&mut peek_nth(peek_nth(r#"="""#.chars())))
        );
    }

    #[test]
    fn attribute_2() {
        assert_eq!(
            Ok(Attribute {
                key: "a".to_owned(),
                value: Some(vec![])
            }),
            parse_attribute(&mut peek_nth(r#"a="""#.chars()))
        );
    }

    #[test]
    fn attribute_3() {
        assert_eq!(
            Ok(Attribute {
                key: "a".to_owned(),
                value: Some(vec![AttributeValuePart::Literal("test".to_owned()),])
            }),
            parse_attribute(&mut peek_nth(r#"a="test""#.chars()))
        );
    }

    #[test]
    fn attributes_1() {
        assert_eq!(Ok(vec![]), parse_attributes(&mut peek_nth("".chars())));
    }

    #[test]
    fn attributes_2() {
        assert_eq!(
            Ok(vec![Attribute {
                key: "a".to_owned(),
                value: Some(vec![AttributeValuePart::Literal("test".to_owned()),])
            }]),
            parse_attributes(&mut peek_nth(r#"a="test""#.chars()))
        );
    }

    #[test]
    fn attributes_3() {
        assert_eq!(
            Ok(vec![
                Attribute {
                    key: "a".to_owned(),
                    value: Some(vec![AttributeValuePart::Literal("test".to_owned()),])
                },
                Attribute {
                    key: "b".to_owned(),
                    value: Some(vec![AttributeValuePart::Literal("jo".to_owned()),])
                }
            ]),
            parse_attributes(&mut peek_nth("a=\"test\" \n\tb=\"jo\"".chars()))
        );
    }

    #[test]
    fn children_1() {
        assert_eq!(Ok(vec![]), parse_children(&mut peek_nth("".chars())));
    }

    #[test]
    fn children_2() {
        assert_eq!(
            Ok(vec![Child::Literal("abc".to_owned())]),
            parse_children(&mut peek_nth("abc".chars()))
        );
    }

    #[test]
    fn children_3() {
        assert_eq!(
            Ok(vec![
                Child::Literal("abc".to_owned()),
                Child::Variable("def".to_owned())
            ]),
            parse_children(&mut peek_nth("abc{{def}}".chars()))
        );
    }

    #[test]
    fn children_4() {
        assert_eq!(
            Ok(vec![
                Child::Literal("abc".to_owned()),
                Child::Variable("def".to_owned()),
                Child::Element(Element {
                    name: "a".to_owned(),
                    attributes: Vec::new(),
                    children: Vec::new(),
                })
            ]),
            parse_children(&mut peek_nth("abc{{def}}<a></a>".chars()))
        );
    }

    #[test]
    fn children_5() {
        assert_eq!(
            Ok(vec![Child::Element(Element {
                name: "a".to_owned(),
                attributes: Vec::new(),
                children: vec![
                    Child::Literal("abc".to_owned()),
                    Child::Variable("def".to_owned()),
                ],
            })]),
            parse_children(&mut peek_nth("<a>abc{{def}}</a>".chars()))
        );
    }

    #[test]
    fn element_1() {
        assert_eq!(
            Ok(Element {
                name: "a".to_owned(),
                attributes: Vec::new(),
                children: vec![
                    Child::Literal("abc".to_owned()),
                    Child::Variable("def".to_owned()),
                ],
            }),
            parse_element(&mut peek_nth("<a>abc{{def}}</a>".chars()))
        );
    }

    #[test]
    fn element_2() {
        assert_eq!(
            Ok(Element {
                name: "a".to_owned(),
                attributes: Vec::new(),
                children: vec![
                    Child::Literal("abc".to_owned()),
                    Child::Variable("def".to_owned()),
                ],
            }),
            parse_element(&mut peek_nth("<a >abc{{def}}</a>".chars()))
        );
    }

    #[test]
    fn element_3() {
        assert_eq!(
            Ok(Element {
                name: "a".to_owned(),
                attributes: vec![Attribute {
                    key: "a".to_owned(),
                    value: Some(vec![AttributeValuePart::Literal("hi".to_owned())])
                }],
                children: vec![],
            }),
            parse_element(&mut peek_nth(r#"<a a="hi"></a>"#.chars()))
        );
    }

    #[test]
    fn partial_block_1() {
        assert_eq!(
            Ok((
                "partial_name".to_owned(),
                vec![Child::Literal("children".to_owned())]
            )),
            parse_partial_block(&mut peek_nth(
                "{{#>partial_name}}children{{/partial_name}}".chars()
            ))
        );
    }

    #[test]
    fn children_with_partial_block() {
        assert_eq!(
            Ok(vec![
                Child::Literal("a".to_owned()),
                Child::PartialBlock(
                    "partial_name".to_owned(),
                    vec![Child::Literal("children".to_owned())]
                ),
                Child::Literal("b".to_owned()),
            ]),
            parse_children(&mut peek_nth(
                "a{{#>partial_name}}children{{/partial_name}}b".chars()
            ))
        );
    }

    #[test]
    fn partial_block_partial_1() {
        assert_eq!(
            Ok(()),
            parse_partial_block_partial(&mut peek_nth("{{>@partial-block}}".chars()))
        );
    }

    #[test]
    fn children_with_partial_block_partial() {
        assert_eq!(
            Ok(vec![
                Child::Literal("a".to_owned()),
                Child::PartialBlockPartial,
                Child::Literal("b".to_owned()),
            ]),
            parse_children(&mut peek_nth("a{{>@partial-block}}b".chars()))
        );
    }
}
