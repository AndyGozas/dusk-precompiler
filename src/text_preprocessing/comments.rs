use crate::code_reference::CharRef;
use crate::{ElementReference, Element};
use crate::warn;

// Function that removes the comments from the given text and moves the
// string contents into a separate variable, leaving a \0 char in it's
// place and remembering the position of each string
pub fn delete_comments (
    input_text: &Vec<CharRef>,
    result_text: &mut Vec<CharRef>,
    static_strings: &mut Vec<String>,
    ) {

    let mut status: char = ' ';
    // ' ' = just normal code
    // any quote sign = a string limited by these quotes
    // / = a single line comment
    // * = a multiline comment

    let mut tmp_static_string: String = "".to_string();

    for symbol_number in 0..input_text.len() {
        match status {
            ' ' => match input_text[symbol_number].value {
                '/'  => {
                    if symbol_number < input_text.len() - 1 {
                        match input_text[symbol_number + 1].value {
                            '/' | '*' => status = input_text[symbol_number + 1].value,
                            _   => result_text.push(input_text[symbol_number].clone()),
                        }
                    } else {
                        result_text.push(input_text[symbol_number].clone());
                    }
                },
                '"' | '\'' | '`'  => {
                    status = input_text[symbol_number].value;
                    result_text.push(input_text[symbol_number].clone());
                    // Store the number of the string in the vector in the
                    // char reference we replace it with
                    result_text.push(CharRef::new(
                            '\0',
                            static_strings.len(),
                            0,
                            input_text[symbol_number].origin.clone(),
                            ));
                },
                _    => result_text.push(input_text[symbol_number].clone()),
            },
            '/' => match input_text[symbol_number].value {
                '\n' => {
                    status = ' ';
                    result_text.push(input_text[symbol_number].clone());
                },
                _    => (),
            },
            '*' => match input_text[symbol_number].value {
                '/' => if input_text[symbol_number - 1].value == '*' {
                    status = ' ';
                    result_text.push(CharRef::new(
                            ' ',
                            input_text[symbol_number].line,
                            input_text[symbol_number].pos,
                            input_text[symbol_number].origin.clone(),
                            ));
                },
                _   => (),
            },
            '"' | '`' | '\'' => {
                if input_text[symbol_number].value == status {
                    if input_text[symbol_number - 1].value != '\\' {
                        status = ' ';
                        result_text.push(input_text[symbol_number].clone());
                        static_strings.push(tmp_static_string);
                        tmp_static_string = "".to_string();
                        continue;
                    }
                }
                tmp_static_string.push(input_text[symbol_number].value);
            },
            _   => print!("{}", status),
        }
    }
    match status {
        '*' => {
            warn::error(
                ElementReference{
                    first: input_text[input_text.len() - 1].clone(),
                    last: input_text[input_text.len() - 1].clone(),
                    element: Element::Object {
                        value: Box::new(0),
                    },
                },
                "E0001",
                "Reached the end of the file without closing the multiline comment",
                "A multiline comment was never closed until the end of file",
                );
            warn::exit(&input_text[input_text.len() - 1].origin, "[E0001]");
        },
        '"' | '\'' | '`' => {
            warn::error(
                ElementReference{
                    first: input_text[input_text.len() - 1].clone(),
                    last: input_text[input_text.len() - 1].clone(),
                    element: Element::Object {
                        value: Box::new(0),
                    },
                },
                "E0000",
                "Reached the end of the file without closing the quote",
                "A quote was never closed until the end of file",
                );
            warn::exit(&input_text[input_text.len() - 1].origin, "[E0000]");
        },
        _ => (),
    }
}
