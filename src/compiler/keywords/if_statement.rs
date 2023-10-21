use crate::compiler::{compiler::Output, keywords::call::parse_keyword_call };
use crate::compiler::symbols::{symbol_to_str, parse_symbols};
use crate::lexer::lexer::*;
use crate::errors::error_msg::*;
use crate::lexer::tokens::*;

use super::jsfunc::InlinedFuncs;
const INIT_ERROR_MISSING_TOKEN: &str =  "Expected a variable name after the keyword init.";
// init a = 0 if (b == 2);
pub fn parse_keyword_if(output: &mut Output, lexer: &Lexer, inline_funcs: &mut InlinedFuncs, mut lexer_index: usize) -> usize {
    output.js_output.push_str("if (");
    lexer_index += 2;
    lexer_index =  parse_args(output, lexer, inline_funcs, lexer_index) + 2;
    output.js_output.push_str(") {");
    lexer_index
}
fn parse_args(output: &mut Output, lexer: &Lexer, inline_funcs: &mut InlinedFuncs, mut lexer_index: usize) -> usize {
    let mut parenthesis_count = 1;
    while lexer_index < lexer.tokens.len() {
        let token = &lexer.tokens[lexer_index];
        match token.symbol_id {
            TOKEN_LPARENT => { parenthesis_count += 1; },
            TOKEN_RPARENT => { parenthesis_count -= 1; },
            // subtract one because we will add one after the function is called
            KEYWORD_CALL => { lexer_index = parse_keyword_call(output, lexer, inline_funcs, lexer_index) - 1; }
            0 => { output.js_output.push_str(&token.text); },
            _ => { output.js_output.push_str(symbol_to_str(token.symbol_id)); }
        }
        if parenthesis_count == 0 { return lexer_index; }
        lexer_index += 1;
        }
    panic!()
}
