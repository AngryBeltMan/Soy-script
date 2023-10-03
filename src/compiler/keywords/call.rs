use crate::errors::error_msg::*;
use crate::{lexer::lexer::*, compiler::compiler::Output};
use crate::lexer::tokens::*;
pub fn parse_keyword_call(output: &mut Output, lexer: &Lexer, mut lexer_index: usize) -> usize {
    lexer_index += 1;
    let call_name = lexer.tokens.get(lexer_index).unwrap_or_error(CompilerError::ExpectedToken, "expected token after keyword call.");
    assert!(call_name.token_type == TokenType::Ident, "ERROR: expected ident for function call.");
    output.js_output.push_str(&call_name.text);
    output.js_output.push('(');
    lexer_index += 1;
    parse_args(lexer, &mut lexer_index, output);
    output.js_output.push_str(")\n");
    // add two because we have to account for the right parenthesis which is not accounted for in
    // the parse_args() function this is because we return early
    lexer_index + 2
}
fn parse_args(lexer: &Lexer, lexer_index: &mut usize, output: &mut Output) {
    while *lexer_index < lexer.tokens.len()  {
        match lexer.tokens[*lexer_index].symbol_id {
            // if it is a comma or left parenthesis we do nothing and continue parsing
            TOKEN_COMMA | TOKEN_LPARENT => {},
            // we end if we encounter a right parenthesis
            TOKEN_RPARENT => return,
            KEYWORD_IF => {},
            KEYWORD_CALL => {
                *lexer_index = parse_keyword_call(output, lexer, *lexer_index) - 1;
            },
            // this means there is not args to the function and can end early
            TOKEN_EMPTYPARENT => {
                *lexer_index -= 1;
                return;
            },
            0 => {
                output.js_output.push_str(&lexer.tokens[*lexer_index].text);
                // pushes a comma so the next argument will be seperated by commas
                // this will lead to trailing commas but that is okay
                output.js_output.push_str(", ");
            }
            id => panic!("ERROR: unknown token id {id}")
        }
        *lexer_index += 1;
    }
}
