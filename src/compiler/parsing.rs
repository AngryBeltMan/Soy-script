// contains tools for parsing the lexer of tokens
use crate::lexer::lexer::Lexer;
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
pub fn get_scope_tokens() {}
