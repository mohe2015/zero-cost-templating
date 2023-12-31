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

pub fn expect_str<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
    expected_str: &str,
) -> Result<(), String> {
    for character in expected_str.chars() {
        expect(input, character)?;
    }
    Ok(())
}

pub fn parse_variable<I: Iterator<Item = char>>(input: &mut PeekNth<I>) -> Result<String, String> {
    let mut inner = || {
        match input.peek_nth(0) {
            Some('{') => {}
            _ => return Err("expected {".to_owned()),
        }
        match input.peek_nth(1) {
            Some('{') => {}
            _ => return Err("expected {".to_owned()),
        }
        let mut identifier = String::new();
        let mut index = 2;
        match input.peek_nth(index) {
            Some('#') => {
                return Err(
                    "expected first character of variable identifier but found # indicating start \
                     of each or if directive"
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
            Some(byte) => identifier.push(*byte),
            None => {
                return Err("expected variable identifier but found end of input".to_owned());
            }
        }
        index += 1;
        loop {
            match input.peek_nth(index) {
                Some('}') => break,
                Some(byte) => {
                    identifier.push(*byte);
                }
                None => {
                    return Err("expected }} but found end of input".to_owned());
                }
            }
            index += 1;
            if identifier == "else" {
                // TODO FIXME we could stop peeking from here on
                // as we know that this is a variable now
                return Err("expected variable identifier but found keyword else".to_owned());
            }
        }
        expect_str(input, "{{")?;
        expect_str(input, &identifier)?;
        expect_str(input, "}}")?;
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
        expect_str(input, "{{#each ")?;
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
        expect_str(input, "{{/each}}")?;
        Ok((identifier, children))
    };
    inner().map_err(|err| format!("{err}\nwhile parsing each"))
}

pub fn parse_if_else<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<(String, Vec<Child>, Vec<Child>), String> {
    let mut inner = || {
        expect_str(input, "{{#if ")?;
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
        let if_children = parse_children(input)?;
        expect_str(input, "{{")?;
        match input.next() {
            Some('/') => {
                expect_str(input, "if}}")?;
                Ok((identifier, if_children, Vec::new()))
            }
            Some('e') => {
                expect_str(input, "lse}}")?;
                let else_children = parse_children(input)?;
                expect_str(input, "{{/if}}")?;
                Ok((identifier, if_children, else_children))
            }
            _ => Err("expected }} but found end of input".to_owned()),
        }
    };
    inner().map_err(|err| format!("{err}\nwhile parsing each"))
}

pub fn parse_partial_block<I: Iterator<Item = char>>(
    input: &mut PeekNth<I>,
) -> Result<(String, Vec<Child>), String> {
    // https://handlebarsjs.com/guide/partials.html#partial-blocks
    let mut inner = || {
        expect_str(input, "{{#>")?;
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
        expect_str(input, "{{/")?;
        expect_str(input, &partial_name)?;
        expect_str(input, "}}")?;
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
    If(String, Vec<Child>, Vec<Child>),
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
                                Some(&'>') => {
                                    // partial block (which may contain inner elements)
                                    let (partial_name, children) = parse_partial_block(input)?;
                                    result.push(Child::PartialBlock(partial_name, children));
                                }
                                Some(&'e') => {
                                    let (identifier, children) = parse_each(input)?;
                                    result.push(Child::Each(identifier, children));
                                }
                                Some(&'i') => {
                                    let (identifier, if_children, else_children) =
                                        parse_if_else(input)?;
                                    result.push(Child::If(identifier, if_children, else_children));
                                }
                                _ => {
                                    return Err("expected partial block, each or if.".to_owned());
                                }
                            }
                        }
                        Some('e') => match input.peek_nth(3) {
                            Some('l') => match input.peek_nth(4) {
                                Some('s') => match input.peek_nth(5) {
                                    Some('e') => match input.peek_nth(6) {
                                        Some('}') => return Ok(result),
                                        _ => result.push(Child::Variable(parse_variable(input)?)),
                                    },
                                    _ => result.push(Child::Variable(parse_variable(input)?)),
                                },
                                _ => result.push(Child::Variable(parse_variable(input)?)),
                            },
                            _ => result.push(Child::Variable(parse_variable(input)?)),
                        },
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
        match name.to_ascii_lowercase().as_str() {
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
    use core::str::Chars;

    use itertools::{peek_nth, PeekNth};

    use crate::html_recursive_descent::{
        parse_attribute, parse_attribute_value, parse_attributes, parse_children, parse_element,
        parse_if_else, parse_partial_block, parse_partial_block_partial, parse_variable, Attribute,
        AttributeValuePart, Child, Element,
    };

    fn fully_parsed<T, F: for<'a> Fn(&'a mut PeekNth<Chars<'static>>) -> Result<T, String>>(
        func: F,
        input: &'static str,
    ) -> Result<T, String> {
        let iterator = &mut peek_nth(input.chars());
        let result = func(iterator);
        if result.is_ok() {
            assert_eq!(None, iterator.next(), "Input not fully parsed");
        }
        result
    }

    #[test]
    fn variable_1() {
        // TODO FIXME check no input left
        assert_eq!(
            Ok("test".to_owned()),
            fully_parsed(parse_variable, "{{test}}")
        );
    }

    #[test]
    fn variable_2() {
        assert_eq!(
            Err("expected } but found end of input\nwhile parsing variable".to_owned()),
            fully_parsed(parse_variable, "{{}}")
        );
    }

    #[test]
    fn variable_3() {
        assert_eq!(
            Err("expected } but found end of input\nwhile parsing variable".to_owned()),
            fully_parsed(parse_variable, "{{test}")
        );
    }

    #[test]
    fn variable_4() {
        assert_eq!(
            Err("expected }} but found end of input\nwhile parsing variable".to_owned()),
            fully_parsed(parse_variable, "{{test")
        );
    }

    #[test]
    fn variable_5() {
        assert_eq!(
            Err(
                "expected variable identifier but found end of input\nwhile parsing variable"
                    .to_owned()
            ),
            fully_parsed(parse_variable, "{{")
        );
    }

    #[test]
    fn variable_6() {
        assert_eq!(
            Err("expected {\nwhile parsing variable".to_owned()),
            fully_parsed(parse_variable, "{")
        );
    }

    #[test]
    fn variable_7() {
        assert_eq!(
            Err("expected {\nwhile parsing variable".to_owned()),
            fully_parsed(parse_variable, "")
        );
    }

    #[test]
    fn variable_8() {
        assert_eq!(
            Err(
                "expected variable identifier but found keyword else\nwhile parsing variable"
                    .to_owned()
            ),
            fully_parsed(parse_variable, "{{else}}")
        );
    }

    #[test]
    fn attribute_value_1() {
        assert_eq!(Ok(vec![]), fully_parsed(parse_attribute_value, ""));
    }

    #[test]
    fn attribute_value_2() {
        assert_eq!(
            Ok(vec![AttributeValuePart::Literal("test".to_owned())]),
            fully_parsed(parse_attribute_value, "test")
        );
    }

    #[test]
    fn attribute_value_3() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Variable("a".to_owned()),
                AttributeValuePart::Literal("test".to_owned())
            ]),
            fully_parsed(parse_attribute_value, "{{a}}test")
        );
    }

    #[test]
    fn attribute_value_4() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Literal("test".to_owned()),
                AttributeValuePart::Variable("a".to_owned()),
            ]),
            fully_parsed(parse_attribute_value, "test{{a}}")
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
            fully_parsed(parse_attribute_value, "{{a}}test{{b}}")
        );
    }

    #[test]
    fn attribute_value_6() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Literal("a".to_owned()),
                AttributeValuePart::Variable("test".to_owned()),
            ]),
            fully_parsed(parse_attribute_value, "a{{test}}")
        );
    }

    #[test]
    fn attribute_value_7() {
        assert_eq!(
            Ok(vec![
                AttributeValuePart::Variable("test".to_owned()),
                AttributeValuePart::Literal("a".to_owned()),
            ]),
            fully_parsed(parse_attribute_value, "{{test}}a")
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
            fully_parsed(parse_attribute_value, "a{{test}}b")
        );
    }

    #[test]
    fn attribute_value_9() {
        assert_eq!(
            Ok(vec![AttributeValuePart::Variable("test".to_owned()),]),
            fully_parsed(parse_attribute_value, "{{test}}")
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
            fully_parsed(parse_attribute, r#"="""#)
        );
    }

    #[test]
    fn attribute_2() {
        assert_eq!(
            Ok(Attribute {
                key: "a".to_owned(),
                value: Some(vec![])
            }),
            fully_parsed(parse_attribute, r#"a="""#)
        );
    }

    #[test]
    fn attribute_3() {
        assert_eq!(
            Ok(Attribute {
                key: "a".to_owned(),
                value: Some(vec![AttributeValuePart::Literal("test".to_owned()),])
            }),
            fully_parsed(parse_attribute, r#"a="test""#)
        );
    }

    #[test]
    fn attributes_1() {
        assert_eq!(Ok(vec![]), fully_parsed(parse_attributes, ""));
    }

    #[test]
    fn attributes_2() {
        assert_eq!(
            Ok(vec![Attribute {
                key: "a".to_owned(),
                value: Some(vec![AttributeValuePart::Literal("test".to_owned()),])
            }]),
            fully_parsed(parse_attributes, r#"a="test""#)
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
            fully_parsed(parse_attributes, "a=\"test\" \n\tb=\"jo\"")
        );
    }

    #[test]
    fn children_1() {
        assert_eq!(Ok(vec![]), fully_parsed(parse_children, ""));
    }

    #[test]
    fn children_2() {
        assert_eq!(
            Ok(vec![Child::Literal("abc".to_owned())]),
            fully_parsed(parse_children, "abc")
        );
    }

    #[test]
    fn children_3() {
        assert_eq!(
            Ok(vec![
                Child::Literal("abc".to_owned()),
                Child::Variable("def".to_owned())
            ]),
            fully_parsed(parse_children, "abc{{def}}")
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
            fully_parsed(parse_children, "abc{{def}}<a></a>")
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
            fully_parsed(parse_children, "<a>abc{{def}}</a>")
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
            fully_parsed(parse_element, "<a>abc{{def}}</a>")
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
            fully_parsed(parse_element, "<a >abc{{def}}</a>")
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
            fully_parsed(parse_element, r#"<a a="hi"></a>"#)
        );
    }

    #[test]
    fn partial_block_1() {
        assert_eq!(
            Ok((
                "partial_name".to_owned(),
                vec![Child::Literal("children".to_owned())]
            )),
            fully_parsed(
                parse_partial_block,
                "{{#>partial_name}}children{{/partial_name}}"
            )
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
            fully_parsed(
                parse_children,
                "a{{#>partial_name}}children{{/partial_name}}b"
            )
        );
    }

    #[test]
    fn partial_block_partial_1() {
        assert_eq!(
            Ok(()),
            fully_parsed(parse_partial_block_partial, "{{>@partial-block}}")
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
            fully_parsed(parse_children, "a{{>@partial-block}}b")
        );
    }

    #[test]
    fn if_else_1() {
        assert_eq!(
            Ok((
                "author".to_owned(),
                vec![Child::Literal("true".to_owned()),],
                vec![]
            )),
            fully_parsed(parse_if_else, "{{#if author}}true{{/if}}")
        );
    }

    #[test]
    fn if_else_2() {
        assert_eq!(
            Ok((
                "author".to_owned(),
                vec![Child::Literal("true".to_owned()),],
                vec![Child::Literal("false".to_owned()),]
            )),
            fully_parsed(parse_if_else, "{{#if author}}true{{else}}false{{/if}}")
        );
    }

    #[test]
    fn if_else_3() {
        assert_eq!(
            Ok(vec![Child::If(
                "author".to_owned(),
                vec![Child::Literal("true".to_owned()),],
                vec![]
            )]),
            fully_parsed(parse_children, "{{#if author}}true{{/if}}")
        );
    }

    #[test]
    fn if_else_4() {
        assert_eq!(
            Ok(vec![Child::If(
                "author".to_owned(),
                vec![Child::Literal("true".to_owned()),],
                vec![Child::Literal("false".to_owned()),]
            )]),
            fully_parsed(parse_children, "{{#if author}}true{{else}}false{{/if}}")
        );
    }
}
