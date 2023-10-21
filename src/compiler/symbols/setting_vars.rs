use crate::lexer::lexer::*;
use crate::errors::error_msg::*;
use crate::Output;

const SETTINGEXPECTEDIDENT: &str = "ERROR: Expected ident after token at sign";
// parses the value after the @ symbol and pushes to the token vector
pub fn parse_setting_var(lexer: &mut Lexer,output: &mut Output, index: usize ) -> usize {
    let ident = lexer.tokens.get(index + 1).unwrap_or_error(CompilerError::ExpectedIdent, SETTINGEXPECTEDIDENT );
    lexer.setting_vars.vars.push(ident.text.clone());
    return index + 2;
}
