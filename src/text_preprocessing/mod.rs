mod comments;
mod parentheses;
mod unspace;

use crate::code_reference::{CodeLines, CharRef};
use unescape::unescape;

// Precompilation process
pub fn precompile (
    name: &String,
    input_string: &String,
    code_lines: &mut std::rc::Rc<CodeLines>,
    static_strings: &mut Vec<String>,
    result_code: &mut Vec<CharRef>,
    ) {

    // Some variables to be used in the future
    *code_lines = std::rc::Rc::new(CodeLines::new(name, input_string));
    let input_code: Vec<CharRef> = CodeLines::to_char_ref(code_lines.clone());
    let mut uncommented_code: Vec<CharRef> = Vec::new();
    let mut static_strings_escaped: Vec<String> = Vec::new();

    // First delete the comments and put all strings away
    // so that their values don't mess anything up later on
    comments::delete_comments(
        &input_code,
        &mut uncommented_code,
        &mut static_strings_escaped,
        );

    // Get rid of all unneeded spaces and newlines and
    // change positions of strings, removed on the previous
    // step accordingly
    unspace::unspace(&uncommented_code, result_code);

    // Check if the parentheses are correct
    parentheses::chck_parentheses(&result_code);

    for st_string in static_strings_escaped {
        static_strings.push(unescape(&st_string).expect("Unknown escape sequence"));
    }
}
