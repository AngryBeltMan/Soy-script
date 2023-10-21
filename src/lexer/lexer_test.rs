use crate::lexer::lexer::*;
#[test]
fn init_var_test() {
    let file = "init hello = 900";
    let lexer = Lexer::parse_str(file);
    let keyword = Token {
        symbol_id: 100,
        text: String::new(),
        token_type: TokenType::Keyword,
    };
    assert!(lexer.tokens[0] == keyword);
}

#[test] fn function_call_test() {
    let function_call = "call hello(arg1,arg2,arg3)";
    let lexer = Lexer::parse_str(function_call);
    eprintln!("{:#?}", lexer);
}

#[test]
fn if_statement_test() {
    let _function_call = "if (a == false) => {;}";
    let function_call = "{}-=";
    let _lexer = Lexer::parse_str(function_call);
    // panic!("{:#?}", lexer);
}

#[test]
fn string_create_test() {
    let function_call = "init a = \"hello world\"";
    let lexer = Lexer::parse_str(function_call);
    eprintln!("{:#?}", lexer.tokens);
    assert_eq!(&lexer.tokens[3].text, "\"hello world\"" );
}
#[test]
fn comments_test() {
    let input = "~this is ignored\n \"and this is not\"";
    let lexer = Lexer::parse_str(input);
    let output = "\"and this is not\"";
    assert_eq!(&lexer.tokens[0].text , output)
}

// checks to see if special quote "`" parses itas a string and not as tokens
#[test]
fn special_quote_test() {
    let input = "`hello = \"world\"` call hello()";
    let lexer = Lexer::parse_str(input);
    eprintln!("wow {:#?}", lexer.tokens);
    assert_eq!(&lexer.tokens[0].text , "hello = \"world\"")
}

