use crate::errors::error_msg::*;
use crate::{lexer::lexer::*, compiler::compiler::Output};
use crate::lexer::tokens::*;
const EXPECTED_TOKEN: &str = "Expected token while parsing keyword func.";
pub fn parse_keyword_func(output: &mut Output, lexer: &Lexer, mut lexer_index: usize) -> usize {
    lexer_index += 1;
    let function_name = &lexer.tokens.get(lexer_index).unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_TOKEN).text;
    lexer_index += 1;
    output.js_output.push_str(&format!("function {function_name}("));
    parse_args(lexer, &mut lexer_index, output);
    output.js_output.push_str(&format!("){{\n"));
    lexer_index + 2
}

pub fn parse_args(lexer: &Lexer, lexer_index: &mut usize, output: &mut Output) {
    while *lexer_index < lexer.tokens.len()  {
        match lexer.tokens[*lexer_index].symbol_id {
            // if it is a comma or left parenthesis we do nothing and continue parsing
            TOKEN_COMMA | TOKEN_LPARENT => {},
            // we end if we encounter a right parenthesis
            TOKEN_RPARENT => return,
            // this means there is not args to the function and can end early
            TOKEN_EMPTYPARENT =>  {return; },
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
