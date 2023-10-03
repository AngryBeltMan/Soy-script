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
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    output.end();
    assert_eq!(output.js_output, "let a = 32;\n");

    let input = "init a = \"hello world\"";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    output.end();
    assert_eq!(output.js_output, "let a = \"hello world\";\n");

    let input = "init a = b";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    output.end();
    assert_eq!(output.js_output, "let a = b;\n");
}

#[test]
fn function_call_test() {
    let input = "call foo()";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
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
    let input = "call foo(call hello(12))";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output, "foo(hello(12, )\n)\n");
}

#[test]
fn function_init() {
    let input = "func foo() {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"function foo(){\n}")
}

#[test]
fn function_init_with_args() {
    let input = "func foo(a1,a2) {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"function foo(a1, a2, ){\n}")
}
#[test]
fn if_statement_test() {
    let input = "if (hello == true) {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"if (hello===true) {}")
}

#[test]
fn if_statement_with_args() {
    let input = "if (call hello() == true) {}";
    let lexer = Lexer::parse_str(input);
    let mut output = Output::init();
    parse_lexer(&mut output, lexer);
    eprintln!("{}", output.js_output);
    assert_eq!(output.js_output,"if (hello()\n===true) {}")
}
