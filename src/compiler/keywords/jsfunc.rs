use crate::compiler::{compiler::Output, keywords::call::parse_keyword_call };
use crate::compiler::symbols::{symbol_to_str, parse_symbols};
use crate::lexer::lexer::*;
use crate::errors::error_msg::*;
use crate::lexer::tokens::*;
use std::collections::HashMap;

use super::functions;
const EXPECTED_FNNAME: &str = "Expected ident for jsfunc name.";
const EXPECTED_PARENT: &str = "Expected token type parenthesis after jsfunc name.";
const EXPECTED_BODY: &str = "Expected function body when parsing jsfunc.";

#[derive(Clone, Debug)]
pub struct JsFunc {
    pub body: String,
    pub args: u8
}
#[derive(Clone, Debug)]
pub struct InlinedFuncs {
    // a hashmap that stores all of the inlined functions
    pub inlined_js_funcs: HashMap<String, JsFunc>,
    // a hashmap that stores all of js macros functions
    pub js_macros: HashMap<&'static str, JsFunc>
}
impl InlinedFuncs {
    pub fn new() -> Self {
        Self { inlined_js_funcs: HashMap::new(), js_macros: HashMap::new() }
    }
}
// jsfunc hello(args) `wow(args)`
pub fn parse_keyword_jsfunc(output: &mut Output, lexer: &Lexer, inline_funcs: &mut InlinedFuncs, mut lexer_index: usize) -> usize {
    // &String because &str is compadible likely because the input for method contains for Vec<T> is &T
    // This checks if the user inlined the function
    if lexer.setting_vars.vars.contains(&String::from("inline")) {
        return parse_js_func_inlined(output, lexer, inline_funcs, lexer_index);
    }
    let name = lexer.tokens.get(lexer_index + 1).unwrap_or_error(CompilerError::ExpectedIdent, EXPECTED_FNNAME);
    let next = lexer.tokens.get(lexer_index + 2).unwrap_or_error(CompilerError::ExpectedSymbol, EXPECTED_PARENT);
    output.js_output.push_str(&format!("function {} (",name.text));
    match next.symbol_id {
        TOKEN_LPARENT => {
            lexer_index += 2;
            // a left parenthesis means there are also other arguments
            functions::parse_args(lexer, &mut lexer_index, output);
        },
        TOKEN_EMPTYPARENT => { lexer_index += 2; }
        _ => compiler_panic(CompilerError::ExpectedSymbol, EXPECTED_PARENT)
    }
    let func_contents = lexer.tokens.get(lexer_index + 1).unwrap_or_error(CompilerError::ExpectedIdent, EXPECTED_BODY );
    eprintln!("func con {}", func_contents.text);
    // the inner code inside the jsfunc
    // we don't increment the lexer index because the special quote "`" that stores the function's
    // body is not parsed/tokenized by the lexer
    output.js_output.push_str(&format!(") {{\n{}\n}}\n", func_contents.text));
    lexer_index + 2
}
// @inline jsfunc hello(hello) ``
// Inlined function contain many limitations like being unable to input a inlined function into
// another function call, not supporting over 255 arguments as the amount of args is represented as a
// u8, and having recursive inline calls
fn parse_js_func_inlined(output: &mut Output, lexer: &Lexer, inline_funcs: &mut InlinedFuncs, mut lexer_index: usize) -> usize {
    let name = lexer.tokens.get(lexer_index + 1).unwrap_or_error(CompilerError::ExpectedIdent, EXPECTED_FNNAME);
    let next = lexer.tokens.get(lexer_index + 2).unwrap_or_error(CompilerError::ExpectedSymbol, EXPECTED_PARENT);

    match next.symbol_id {
        // TOKEN_EMPTYPARENT => {
        //     let body = lexer.tokens.get(lexer_index + 3)
        //         .unwrap_or_error(CompilerError::ExpectedIdent, &format!("ERROR: Error while parsing jsfunc {}. Expected body after empty parenthesis", name.text));
        //     inline_funcs.inlined_js_funcs.insert(name.text.to_string(), JsFunc { args: 0, body: body.text.to_string() });
        //     return lexer_index + 6;
        // }
        TOKEN_LPARENT => {
            // add 2 so get_args begins parsing after the left parenthesis
            // this is important or else get_args will parse the function as a function argument
            // which is wrong because it is Okay
            lexer_index += 2;
            let args = get_args(output, lexer, &mut lexer_index, &name.text);
            // get_args ends parsing after encountering a right parenthesis so we must add one to get
            // the next thing after the end of the arguments and by adding one we "hope" to get the
            // body of the jsfunction
            let body = lexer.tokens.get(lexer_index + 1)
                .unwrap_or_error(CompilerError::ExpectedIdent, &format!("ERROR: Error while parsing jsfunc {}. Expected body after left parenthesis. (jsfunc.rs)", name.text));
            let mut new_body = body.text.replace(&format!("@{}", args[0]), &format!("__inline_{}_arg0", name.text) );
            for arg_index in 1..args.len() {
                new_body = new_body.replace(&format!("@{}", args[arg_index]), &format!("__inline_{}_arg{arg_index}", name.text) );
            }
            eprintln!("args {:?}", args);
            eprintln!("body {}", new_body);
            inline_funcs.inlined_js_funcs.insert(name.text.to_string(), JsFunc { args: args.len() as u8, body: new_body });
            return lexer_index + 2;
        }
        TOKEN_EMPTYPARENT => {
            let body = lexer.tokens.get(lexer_index + 3)
                .unwrap_or_error(CompilerError::ExpectedIdent, &format!("ERROR: Error while parsing jsfunc: {}. Expected body after empty parenthesis. (jsfunc.rs)", name.text));
            inline_funcs.inlined_js_funcs.insert(name.text.to_string(), JsFunc { args: 0, body: body.text.to_string() });
            return lexer_index + 4;
        }
        _ => compiler_panic(CompilerError::ExpectedSymbol, &format!( "ERROR: Expected L-Parent when parsing jsfunc: {}, got id {}. (jsfunc.rs)", name.text, next.symbol_id))
    };
}

// parses the arguments in a function and saves them to a vector of strings
// ends parsing once it encounters a left parenthesis ')'
pub fn get_args<'a>(output: &mut Output, lexer: &'a Lexer, lexer_index: &mut usize, func_name: &str) -> Vec<&'a str> {
    let mut args = vec![];
    loop {
        let token = lexer.tokens.get(*lexer_index).unwrap_or_error(CompilerError::ExpectedToken,&format!("ERROR: expected arguments after jsfunc {func_name}"));
        if token.token_type == TokenType::Ident {
            args.push(token.text.as_str());
        }
        if token.symbol_id == TOKEN_RPARENT {
            return args;
        }
        *lexer_index += 1;
    }
}
