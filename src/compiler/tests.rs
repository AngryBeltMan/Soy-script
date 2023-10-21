use crate::compiler::keywords::jsfunc::InlinedFuncs;
use crate::lexer::lexer::Lexer;

use super::compiler::*;
use super::keywords::*;
#[test]
fn init_html_output() {
    let mut output = Output::init();
    output.end();
    eprintln!("{}", output.html_output);
    assert_eq!(&output.html_output.replace('\n', ""), r#"<!DOCTYPE html><html lang="en"><meta charset="utf-8"><script src="init.js"></script></html>"#)
}
#[test]
fn init_variable_output() {
    let input = "init a = 32";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    output.end();
    assert_eq!(output.js_output, "let a = 32;\n");

    let input = "init a = \"hello world\"";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);

    eprintln!("{}", output.js_output);
    output.end();
    assert_eq!(output.js_output, "let a = \"hello world\";\n");

    let input = "init a = b";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    output.end();
    assert_eq!(output.js_output, "let a = b;\n");
}

#[test]
fn function_call_test() {
    let input = "call foo()";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output, "foo()\n")
}

// #[test]
// fn function_call_test_with_args() {
//     let input = "call foo(wow, cool,hello ,awesome,)";
//     let lexer = Lexer::parse_str(input);
//     let mut output = Output::init();
//     parse_lexer(&mut output, lexer);
//     eprintln!("{}", output.js_output);
//     assert_eq!(output.js_output, "foo(wow, cool, hello, awesome, )\n");
// }

#[test]
fn function_call_with_inner_function_call() {
    let input = "call foo(call hello(12),)";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output, "foo(hello(12, )\n)\n");
}

#[test]
fn function_init() {
    let input = "func foo() {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"function foo(){\n}")
}

#[test]
fn function_init_with_args() {
    let input = "func foo(a1,a2) {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"function foo(a1, a2, ){\n}")
}
#[test]
fn if_statement_test() {
    let input = "if (hello == true) {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"if (hello===true) {}")
}

#[test]
fn if_statement_with_args() {
    let input = "if (call hello() == true) {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"if (hello()\n===true) {}")
}

#[test]
fn repeat_lest() {
    let input = "repeat 0,2 { call hello(##) }";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"hello(0, )\nhello(1, )\nhello(2, )\n")
}

#[test]
fn forloop_test() {
    let input = "for i 0 -> 10 {call hello()}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"for (let i = 0; i < 10; ++i;) {\nhello()\n}")
}

#[test]
fn jsfunc_test() {
    let input = "jsfunc add(lhs, rhs) `\nreturn lhs + rhs;`";
    let lexer = Lexer::parse_str(input);
    eprintln!("{:#?}", lexer.tokens);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("output:  {}", output.js_output);
    assert_eq!(output.js_output,"function add (lhs, rhs, ) {\n\nreturn lhs + rhs;\n}\n")
}

#[test]
fn jsfunc_test_no_args() {
    let input = "jsfunc say_hello() `console.log(\"hello\")`";
    let lexer = Lexer::parse_str(input);
    eprintln!("{:#?}", lexer.tokens);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    eprintln!("output:  {}", output.js_output);
    assert_eq!(output.js_output,"function say_hello () {\nconsole.log(\"hello\")\n}\n")
}

// checks to see if the inlined function is stored properly in the inlined function hashmap
// #[test]
// fn inline_jsfunc_test() {
//     let input = "@inline jsfunc add() `\nreturn lhs + rhs;`";
//     let lexer = Lexer::parse_str(input);
//     let mut output = Output::init();
//     parse_lexer(&mut output, lexer, InlinedFuncs::new() );
// }

// checks to see if the inlined function is stored properly in the inlined function hashmap
// #[test]
// fn inline_jsfunc_args_test() {
//     let input = "@inline jsfunc add(lhs, rhs) `\nreturn @lhs + @rhs;` @inline jsfunc print_hello_inlined() `console.log(\"hello\")`";
//     let lexer = Lexer::parse_str(input);
//     let mut output = Output::init();
//     parse_lexer(&mut output, lexer, InlinedFuncs::new() );
//     let body = INLINEJS_FUNCS.lock().unwrap();
//     let body = body.get("add").unwrap();
//     assert_eq!(&body.body, "\nreturn __inline_lhs_arg0 + __inline_rhs_arg1;");
// }
#[test]
fn setting_var_test() {
    let input = "@hello call wow()";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    // assert_eq!(&lexer.setting_vars.vars[0], "hello" );
}
#[test]
fn inline_call_js_func_args() {
    let input = "@inline jsfunc sum(lhs, rhs) `console.log(@lhs + @rhs)`\n call sum(60, 9)";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    assert_eq!(output.js_output,"let __inline_sum_arg0 = 60;\nlet __inline_sum_arg1 = 9;\nconsole.log(__inline_sum_arg0 + __inline_sum_arg1)");
}

#[test]
fn inline_call_js_func_no_args() {
    let input = "@inline jsfunc no_args() `console.log(\"no args!\");`\n call no_args()";
    let lexer = Lexer::parse_str(input);
    // eprintln!("{:#?}", lexer.tokens);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    assert_eq!(output.js_output, "console.log(\"no args!\");");
}
#[test]
fn inline_repeat() {
    let input = "@inline jsfunc no_args() `console.log(\"no args!\");`\n repeat 0,5 {call no_args()}";
    let lexer = Lexer::parse_str(input);
    // eprintln!("{:#?}", lexer.tokens);
    let mut output = Output::init();
    let mut inline_funcs = InlinedFuncs::new();
    parse_lexer(&mut output, lexer,  &mut inline_funcs);
    // console.log(\"no args!\"); six times
    assert_eq!(output.js_output, "console.log(\"no args!\");console.log(\"no args!\");console.log(\"no args!\");console.log(\"no args!\");console.log(\"no args!\");console.log(\"no args!\");");
}
