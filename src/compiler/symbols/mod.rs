use crate::lexer::{lexer::Token, tokens::*};

use super::compiler::Output;

pub fn parse_symbols(token: &Token, output: &mut Output) {
    output.js_output.push_str(symbol_to_str(token.symbol_id));
}

pub fn symbol_to_str<'a>(token_id: u8) -> &'a str {
    match token_id {
        TOKEN_RBRACKET => "}",
        // js struct opperators
        TOKEN_CMPEQUAL => "===",
        TOKEN_GREATER => "<",
        TOKEN_LESS => ">",
        TOKEN_BIGARROWLEFT => "<=",
        TOKEN_LESSEQUAL => ">=",
        TOKEN_EMPTYPARENT => "()",
        TOKEN_LPARENT => "(",
        TOKEN_RPARENT => ")",
        id => panic!("ERROR: Unexpected token id ({id}) while parsing to str")
    }
}
