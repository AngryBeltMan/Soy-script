use crate::{lexer::lexer::{Lexer, TokenType, SettingVars}, compiler::{compiler::{Output, parse_lexer}, parsing::get_scope_tokens}, errors::error_msg::*};

use super::jsfunc::InlinedFuncs;

const EXPECTED_TOKEN_ARG1: &str =  "Expected token while paring first argument for keyword repeat.";
const EXPECTED_IDENT_ARG1: &str =  "ERROR: expected ident while parsing first argument for keyword repeat";
const EXPECTED_TOKEN_ARG2: &str =  "Expected token while paring second argument for keyword repeat.";
const EXPECTED_IDENT_ARG2: &str =  "ERROR: expected ident while parsing second argument for keyword repeat";
const EXPECTED_USIZE: &str =  "ERROR: expected ident while parsing second argument for keyword repeat";
// repeat 0,10 {}
// comma is not nesisary as long as the two numbers are divided
// 0 is the start
// 10 is the amount of times it should repeat
pub fn parse_keyword_repeat(output: &mut Output, lexer: &Lexer, inline_funcs: &mut InlinedFuncs, mut index: usize) -> usize {
    index += 1;
    let start = lexer.tokens.get(index).unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_TOKEN_ARG1)
        .text.parse::<usize>().unwrap_or_error(CompilerError::ExpectedUnsignedInteger, EXPECTED_USIZE);
    index += 2;
    let repeat_amount = lexer.tokens.get(index)
        .unwrap_or_error(CompilerError::ExpectedToken, EXPECTED_TOKEN_ARG2)
        .text.parse::<usize>().unwrap_or_error(CompilerError::ExpectedUnsignedInteger, EXPECTED_USIZE);
    index += 2;
    let mut tokens = get_scope_tokens(lexer, &mut index);
    tokens.pop();
    let mut new_output = Output::init();
    let lexer = Lexer {
        setting_vars: SettingVars {vars: vec![]},
        tokens: tokens.iter().map(|token| token.to_owned().to_owned()).collect()
    };
    parse_lexer(&mut new_output,  lexer, inline_funcs );
    for rep_index in start..=start + repeat_amount {
        output.js_output.push_str(&new_output.js_output.replace("##", &rep_index.to_string()));
    }
    return index;
}
