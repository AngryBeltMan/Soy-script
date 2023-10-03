use crate::compiler::compiler::Output;
use crate::lexer::lexer::*;
use crate::errors::error_msg::*;
use crate::lexer::tokens::*;
const INIT_ERROR_MISSING_TOKEN: &str =  "Expected a variable name after the keyword init.";
// init a = 0 if (b == 2);
pub fn parse_keyword_init(output: &mut Output, lexer: &Lexer, mut lexer_index: usize) -> usize {
    lexer_index += 1;
    let var_name = &lexer.tokens.get(lexer_index)
        .unwrap_or_error(CompilerError::ExpectedIdent, INIT_ERROR_MISSING_TOKEN).text;
    assert!(var_name.len() > 0, "ERROR: expected an ident for variable name." );
    output.js_output.push_str(&format!("let {var_name}"));
    // checks if there is an equal sign
    lexer_index += 1;
    if lexer.tokens[lexer_index].symbol_id == TOKEN_EQUALSIGN {
        lexer_index += 1;
        let var_value = &lexer.tokens.get(lexer_index)
            .unwrap_or_error(CompilerError::ExpectedIdent, INIT_ERROR_MISSING_TOKEN).text;
        output.js_output.push_str(&format!(" = {var_value}"));
    }
    output.js_output.push_str(";\n");
    lexer_index + 1
}
