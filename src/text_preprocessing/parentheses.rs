use crate::code_reference::CharRef;
use crate::{Element, ElementReference};
use crate::warn;

pub fn chck_parentheses (
    code: &Vec<CharRef>,
    ) {

    let mut pattern: Vec<CharRef> = Vec::new();
    for symbol_number in 0..code.len() {
        match code[symbol_number].value {
            '(' | '{' | '[' => pattern.push(code[symbol_number].clone()),
            '<'             => if symbol_number < code.len() - 1 {
                if code[symbol_number + 1].value == ':' {
                    pattern.push(code[symbol_number].clone());
                }
            },
            ')'             => {
                if pattern.len() == 0 {
                    warn::error(
                        ElementReference{
                            first: code[symbol_number].clone(),
                            last: code[symbol_number].clone(),
                            element: Element::Object{
                                value: Box::new(" "),
                            },
                        },
                        "E0002",
                        "Parentheses mismatch",
                        "Trying to close a never opened bracket",
                        );
                    warn::exit(&code[symbol_number].origin, "[E0002]");
                }
                let closed = pattern.pop().unwrap();
                if closed.value != '(' {
                    warn::error(
                        ElementReference{
                            first: closed.clone(),
                            last: code[symbol_number].clone(),
                            element: Element::Object{
                                value: Box::new(" "),
                            },
                        },
                        "E0002",
                        "Parentheses mismatch",
                        "You have either forgotten to close a bracket, did it incorrectly, or tried to close a never opened bracket",
                        );
                    warn::exit(&code[symbol_number].origin, "[E0002]");
                }
            },
            '}'             => {
                if pattern.len() == 0 {
                    warn::error(
                        ElementReference{
                            first: code[symbol_number].clone(),
                            last: code[symbol_number].clone(),
                            element: Element::Object{
                                value: Box::new(" "),
                            },
                        },
                        "E0002",
                        "Parentheses mismatch",
                        "Trying to close a never opened bracket",
                        );
                    warn::exit(&code[symbol_number].origin, "[E0002]");
                }
                let closed = pattern.pop().unwrap();
                if closed.value != '{' {
                    warn::error(
                        ElementReference{
                            first: closed.clone(),
                            last: code[symbol_number].clone(),
                            element: Element::Object{
                                value: Box::new(" "),
                            },
                        },
                        "E0002",
                        "Parentheses mismatch",
                        "You have either forgotten to close a bracket, did it incorrectly, or tried to close a never opened bracket",
                        );
                    warn::exit(&code[symbol_number].origin, "[E0002]");
                }
            },
            ']'             => {
                if pattern.len() == 0 {
                    warn::error(
                        ElementReference{
                            first: code[symbol_number].clone(),
                            last: code[symbol_number].clone(),
                            element: Element::Object{
                                value: Box::new(" "),
                            },
                        },
                        "E0002",
                        "Parentheses mismatch",
                        "Trying to close a never opened bracket",
                        );
                    warn::exit(&code[symbol_number].origin, "[E0002]");
                }
                let closed = pattern.pop().unwrap();
                if closed.value != '[' {
                    warn::error(
                        ElementReference{
                            first: closed.clone(),
                            last: code[symbol_number].clone(),
                            element: Element::Object{
                                value: Box::new(" "),
                            },
                        },
                        "E0002",
                        "Parentheses mismatch",
                        "You have either forgotten to close a bracket, did it incorrectly, or tried to close a never opened bracket",
                        );
                    warn::exit(&code[symbol_number].origin, "[E0002]");
                }
            },
            '>' => {
                if symbol_number > 0 {
                    if code[symbol_number - 1].value == ':' {
                        if pattern.len() == 0 {
                            warn::error(
                                ElementReference{
                                    first: code[symbol_number].clone(),
                                    last: code[symbol_number].clone(),
                                    element: Element::Object{
                                        value: Box::new(" "),
                                    },
                                },
                                "E0002",
                                "Parentheses mismatch",
                                "Trying to close a never opened bracket",
                                );
                            warn::exit(&code[symbol_number].origin, "[E0002]");
                        }
                        let closed = pattern.pop().unwrap();
                        if closed.value != '<' {
                            warn::error(
                                ElementReference{
                                    first: closed.clone(),
                                    last: code[symbol_number].clone(),
                                    element: Element::Object{
                                        value: Box::new(" "),
                                    },
                                },
                                "E0002",
                                "Parentheses mismatch",
                                "You have either forgotten to close a bracket, did it incorrectly, or tried to close a never opened bracket",
                        );
                            warn::exit(&code[symbol_number].origin, "[E0002]");
                        }
                    }
                }
            },
            _ => (),
        }
    }
    if pattern.len() == 0 {
        return();
    }
    let not_closed = pattern.pop().unwrap();
    warn::error(
        ElementReference{
            first: not_closed.clone(),
            last: not_closed.clone(),
            element: Element::Object{
                value: Box::new(" "),
            },
        },
        "E0002",
        "Parentheses mismatch",
        "Bracket never closed until the end of the scope",
        );
    warn::exit(&not_closed.origin, "[E0002]");
}
