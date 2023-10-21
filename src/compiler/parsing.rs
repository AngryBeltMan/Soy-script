// contains tools for parsing the lexer of tokens
use crate::lexer::{lexer::*, tokens::*};
pub fn collect_until(lexer: &Lexer, mut index: usize, end_token_id: u8 ) -> (String, usize) {
    let mut contents = String::new();
    while index < lexer.tokens.len() {
        if lexer.tokens[index].symbol_id == end_token_id {
            return (contents, index)
        } else {

        }
    }
    panic!();
}
pub fn get_scope_tokens<'a>(lexer: &'a Lexer, index: &mut usize) -> Vec<&'a Token> {
    let mut braket_count = 1;
    let mut tokens = vec![];
    while (*index < lexer.tokens.len()) && (braket_count > 0) {
        println!("call {braket_count}");
        let token = &lexer.tokens[*index];
        if token.symbol_id == TOKEN_LBRACKET { braket_count += 1; }
        if token.symbol_id == TOKEN_RBRACKET { braket_count -= 1; }
        tokens.push(token);
        *index += 1;
    }
    return tokens;
}
