use crate::errors::error_msg::*;
use crate::lexer::lexer::*;
use crate::compiler::keywords::*;

use super::symbols::parse_symbols;
const ERRORINDEXINGLEXER: &str = "ERROR: index out of bounds when parsing file.";
#[derive(Clone, Debug)]
pub struct Output {
    pub html_output: String,
    pub js_output: String,
}
impl Output {
    pub fn init() -> Self {
        Self {
            html_output: format!("<!DOCTYPE html>\n<html lang=\"en\">\n<meta charset=\"utf-8\">\n<script src=\"init.js\"></script>\n"),
            js_output: String::new(),
        }
    }
    // should be called after parsing and before writing to the file
    pub fn end(&mut self) {
        self.html_output.push_str("</html>");
    }
}
pub fn parse_lexer(output: &mut Output, lexer: Lexer) {
    let mut index = 0;
    while index < lexer.tokens.len() {
        let token =  lexer.tokens.get(index)
            .unwrap_or_error(CompilerError::ExpectedToken, ERRORINDEXINGLEXER );
        match token.token_type {
            TokenType::Keyword => match_keyword(&lexer, output, &mut index, token.symbol_id) ,
            TokenType::SingleSymbol | TokenType::DoubleSymbol => {
                parse_symbols(token, output);
                index += 1;
            },

            _ => panic!("unknown token type {:#?}", token)
        };
    }
}