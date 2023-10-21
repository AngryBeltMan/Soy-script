use crate::compiler::compiler::Output;
use crate::lexer::lexer::*;
use crate::errors::error_msg::*;
use crate::lexer::tokens::*;
const EXPECTED_VAR: &str = "Expected a variable while parsing the keyword for.";
const EXPECTED_ARROW: &str = "Expected token arrow while parsing the keyword for.";
const EXPECTED_START: &str = "Expected an integer/var while parsing the start range keyword for.";
const EXPECTED_END: &str = "Expected an integer/var while parsing the end range keyword for.";
// for i 0
//
pub fn parse_keyword_for(output: &mut Output, lexer: &Lexer, mut lexer_index: usize) -> usize {
    let for_loop_var = &lexer.tokens.get(lexer_index + 1)
        .unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_VAR).text;
    compiler_assert(for_loop_var.len() > 0, CompilerError::ExpectedToken, EXPECTED_VAR);

    let start = &lexer.tokens.get(lexer_index + 2)
        .unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_START).text;
    compiler_assert(start.len() > 0, CompilerError::ExpectedToken, EXPECTED_START);

    let direction = lexer.tokens.get(lexer_index + 3)
        .unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_ARROW);

    let end = &lexer.tokens.get(lexer_index + 4)
        .unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_END).text;
    compiler_assert(end.len() > 0, CompilerError::ExpectedToken, EXPECTED_END);
    match direction.symbol_id {
        TOKEN_THINARROWLEFT => output.js_output.push_str(&format!("for (let {for_loop_var} = {start}; {for_loop_var} > {end}; --{for_loop_var};) {{\n")),
        TOKEN_THINARROWRIGHT => output.js_output.push_str(&format!("for (let {for_loop_var} = {start}; {for_loop_var} < {end}; ++{for_loop_var};) {{\n")),

        TOKEN_BIGARROWLEFT => output.js_output.push_str(&format!("for (let {for_loop_var} = {start}; {for_loop_var} >= {end}; --{for_loop_var};) {{\n")),
        TOKEN_BIGARROWRIGHT => output.js_output.push_str(&format!("for (let {for_loop_var} = {start}; {for_loop_var} <= {end}; ++{for_loop_var};) {{\n")),
        _ => compiler_panic(CompilerError::ExpectedSymbol, EXPECTED_ARROW)
    }
    return lexer_index + 6;
}
