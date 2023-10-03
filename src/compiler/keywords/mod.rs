use crate::lexer::{lexer::Lexer, tokens::*};

use self::init::parse_keyword_init;

use super::compiler::Output;

mod init;
mod if_statement;
mod functions;
mod call;
// matches the keyword type and parses the lexer's token stream based on the keyword
pub fn match_keyword(lexer: &Lexer,output: &mut Output, index: &mut usize, token_id: u8) {
    match token_id {
        KEYWORD_INIT => { *index = parse_keyword_init(output, lexer, *index); },
        KEYWORD_IF => {
            println!("if");
            *index = if_statement::parse_keyword_if(output, lexer, *index) },
        KEYWORD_FUNC => { *index = functions::parse_keyword_func(output, lexer, *index); },
        KEYWORD_CALL => { *index = call::parse_keyword_call(output, lexer, *index); },
        _ => unreachable!()
    }
}
