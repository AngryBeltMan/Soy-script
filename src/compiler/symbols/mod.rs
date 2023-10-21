use crate::{lexer::{lexer::*, tokens::*}, errors::error_msg::*};

use super::compiler::*;
mod setting_vars;

pub fn parse_symbols(lexer: &mut Lexer, index: usize, output: &mut Output) -> usize {
    // we can unwrap because we know at this index it will be a token with the type symbol
    let token = lexer.tokens.get(index).unwrap();
    match token.symbol_id {
        // the at symbol "@" will be used to create setting vars and therefore will be treated
        // differently
        TOKEN_ATSIGN => setting_vars::parse_setting_var(lexer, output, index),
        _ => {
            eprintln!("symbol index {index}");
            output.js_output.push_str(symbol_to_str(token.symbol_id));
            return index + 1;
        }
    }
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
