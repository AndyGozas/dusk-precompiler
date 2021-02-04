use crate::code_reference::CharRef;

// Get rid of unneeded spaces and newlines in the code
// making sure not to remove only the ones that don't matter
pub fn unspace(input_text: &Vec<CharRef>, result_text: &mut Vec<CharRef>) {
    let mut status: u8 = 0;
    // 0 = no spaces and delimiters preceeding
    // 1 = the last character is a space
    // 2 = the last character is a delimiter (no additional separation needed)

    let mut tmp_pos: usize;
    // The variable, used for storing the position of
    // the last element in some cases

    for symbol_number in 0..input_text.len() {
        match input_text[symbol_number].value {
            ' ' | '\n' | '\t' => match status {
                // If the current symbol is a space or another empty delimiter
                // then it should be logged into the status variable and if
                // the last symbol is not a delimiter a space should be pushed
                0 => {
                    status = 1;
                    result_text.push(CharRef::new(
                        ' ',
                        input_text[symbol_number].line,
                        input_text[symbol_number].pos,
                        input_text[symbol_number].origin.clone(),
                        ));
                },
                _ => (),
            },
            ';' | ',' | '.' | ':' | '=' | '+' | '-' | '/' | '*' | '"' | '\''
                | '`' | '{' | '}' | '[' | ']' | '(' | ')' | '<' | '>' | '\\'
                | '!' | '?' | '|' | '%' | '&' | '#' | '^' | '~' | '@' | '$'
                => match status {
                    // If the current symbol is an unempty delimiter symbol
                    // then it does not need any additional spaces after it
                    // nor before it, so if the previous symbol is already a
                    // space, the current symbol should be put in it's place
                    0 => {
                        status = 2;
                        result_text.push(input_text[symbol_number].clone());
                    },
                    1 => {
                        status = 2;
                        tmp_pos = result_text.len() - 1;
                        result_text[tmp_pos] = input_text[symbol_number].clone();
                    },
                    2 => result_text.push(input_text[symbol_number].clone()),
                    _ => (),
                },
            _ => {
                // If the current symbol is not a delimiter symbol, then
                // we should just push it into the result and set the
                // status to 0
                result_text.push(input_text[symbol_number].clone());
                status = 0;
            },
        }
    }
}
