use crate::lexer::{lexer::Lexer, tokens::*};

use self::{init::parse_keyword_init, jsfunc::InlinedFuncs};

use super::compiler::Output;

mod init;
mod if_statement;
mod functions;
mod call;
mod repeat;
mod forloops;
pub mod jsfunc;
// matches the keyword type and parses the lexer's token stream based on the keyword
pub fn match_keyword(lexer: &Lexer,output: &mut Output, inline_data: &mut InlinedFuncs, index: &mut usize, token_id: u8) {
    match token_id {
        KEYWORD_INIT => { *index = parse_keyword_init(output, lexer, *index); },
        KEYWORD_IF => { *index = if_statement::parse_keyword_if(output, lexer, inline_data, *index) },
        KEYWORD_FUNC => { *index = functions::parse_keyword_func(output, lexer, *index); },
        KEYWORD_JSFUNC => { *index = jsfunc::parse_keyword_jsfunc(output, lexer, inline_data, *index); },
        KEYWORD_CALL => { *index = call::parse_keyword_call(output, lexer, inline_data, *index); },
        KEYWORD_REPEAT => { *index = repeat::parse_keyword_repeat(output, lexer, inline_data, *index); },
        KEYWORD_FOR => { *index = forloops::parse_keyword_for(output, lexer, *index); },
        _ => unreachable!()
    }
}
