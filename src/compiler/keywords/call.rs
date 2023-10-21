use crate::errors::error_msg::*;
use crate::lexer::tokens::*;
use crate::{compiler::compiler::Output, lexer::lexer::*};

use super::jsfunc::{get_args, InlinedFuncs};
pub fn parse_keyword_call(
    output: &mut Output,
    lexer: &Lexer,
    inline_funcs: &mut InlinedFuncs,
    mut lexer_index: usize,
) -> usize {
    lexer_index += 1;
    let call_name = lexer.tokens.get(lexer_index).unwrap_or_error(
        CompilerError::ExpectedToken,
        "expected token after keyword call (call.rs).",
    );
    // if check inline is true the function being called is inlined and has already been so we can
    // just return right now
    if check_inline_call(
        output,
        lexer,
        inline_funcs,
        &mut lexer_index,
        &call_name.text,
    ) {
        eprintln!("inlined {}", call_name.text);
        eprintln!("returned");
        return lexer_index;
    }
    assert!(
        call_name.token_type == TokenType::Ident,
        "ERROR: expected ident for function call. (call.rs)"
    );
    // pushes the name of the function and then the left parenthesis
    // <name>(<args from function parse_args>)\n
    output.js_output.push_str(&call_name.text);
    output.js_output.push('(');
    lexer_index += 1;
    parse_args(lexer, &mut lexer_index, output, inline_funcs);
    output.js_output.push_str(")\n");

    // add two because we have to account for the right parenthesis which is not accounted for in
    // the parse_args() function this is because we return early
    lexer_index + 2
}
fn parse_args(
    lexer: &Lexer,
    lexer_index: &mut usize,
    output: &mut Output,
    inline_funcs: &mut InlinedFuncs,
) {
    while *lexer_index < lexer.tokens.len() {
        match lexer.tokens[*lexer_index].symbol_id {
            // if it is a comma or left parenthesis we do nothing and continue parsing
            TOKEN_COMMA | TOKEN_LPARENT => {}
            // we end if we encounter a right parenthesis
            TOKEN_RPARENT => return,
            KEYWORD_IF => {}
            KEYWORD_CALL => {
                *lexer_index = parse_keyword_call(output, lexer, inline_funcs, *lexer_index) - 1;
            }
            // this means there is not args to the function and can end early
            TOKEN_EMPTYPARENT => {
                *lexer_index -= 1;
                return;
            }
            0 => {
                output.js_output.push_str(&lexer.tokens[*lexer_index].text);
                // pushes a comma so the next argument will be seperated by commas
                // this will lead to trailing commas but that is okay
                output.js_output.push_str(", ");
            }
            id => panic!("ERROR: unknown token id {id}. (call.rs)"),
        }
        *lexer_index += 1;
    }
}
fn check_inline_call(
    output: &mut Output,
    lexer: &Lexer,
    inline_funcs: &mut InlinedFuncs,
    lexer_index: &mut usize,
    function_name: &str,
) -> bool {
    if !inline_funcs.inlined_js_funcs.contains_key(function_name) { return false; }
    // add one because it will begin the parse args after the parenthesis
    *lexer_index += 1;
    let arg_count =
        parse_args_inlined(lexer, lexer_index, function_name, output, inline_funcs) as u8;
    // should not fail because we check if the key exists t the start
    let func_data = inline_funcs
        .inlined_js_funcs
        .get(function_name)
        .expect("ERROR: call cannot get value. (call.rs)");
    compiler_assert(arg_count == func_data.args, CompilerError::IncorrectAmountOfArgs,
                    &format!("Incorrect amount of args were supplied fo the function {function_name}. {function_name} expected {} arg(s) but was provided with {arg_count}. (call.rs)", func_data.args));
    output.js_output.push_str(&func_data.body);
    // add one to parse after right parenthesis
    *lexer_index += 1;
    return true;
}
// parses the args of an inlined function
// returns the amount of arguments inputed so it can be checked later so see if the right amount of
// arguments is supplied
fn parse_args_inlined(
    lexer: &Lexer,
    lexer_index: &mut usize,
    fn_name: &str,
    output: &mut Output,
    inline_funcs: &mut InlinedFuncs,
) -> usize {
    // holds what current argument indx this is important because we need to know the index of the
    // argument for inlined calls
    let mut arg_index = 0;
    eprintln!("len {}, {lexer_index}", lexer.tokens.len());
    while *lexer_index < lexer.tokens.len() {
        eprintln!("syyymbol {}", lexer.tokens[*lexer_index].symbol_id);
        match lexer.tokens[*lexer_index].symbol_id {
            TOKEN_COMMA | TOKEN_LPARENT => {},
            // we end if we encounter a right parenthesis
            TOKEN_RPARENT => { return arg_index; },
            KEYWORD_IF => todo!(),
            KEYWORD_CALL => {
                // this is how inlined variables are formmatted
                let inline_arg = format!("__inline_{fn_name}_arg{arg_index}");
                // sets the inlined arg equal to the function call
                output.js_output.push_str(&format!("let {inline_arg} = "));
                *lexer_index = parse_keyword_call(output, lexer, inline_funcs, *lexer_index) - 1;
                output.js_output.push('\n');
                arg_index += 1;
            },
            // this means there is not args to the function and can end early
            TOKEN_EMPTYPARENT => {
                *lexer_index += 1;
                return 0;
            },
            0 => {
                let inline_arg = format!("__inline_{fn_name}_arg{arg_index}");
                output.js_output.push_str(&format!("let {inline_arg} = {};\n",&lexer.tokens[*lexer_index].text));
                arg_index += 1;
            }
            id => panic!("ERROR: unknown token id {id} while parsing call args for inlined function {fn_name}. (call.rs)")
        }
        *lexer_index += 1;
    }
    compiler_panic(CompilerError::UnclosedFunction,
                   &format!("Unclosed function expected right parenthesis after calling inlined function: {fn_name} got symbol id {}. (call.rs)", lexer.tokens[*lexer_index - 1].symbol_id));
}
