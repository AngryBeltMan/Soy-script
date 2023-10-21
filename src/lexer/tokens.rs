#![allow(dead_code)]
// single character symbols

// '='
pub const TOKEN_EQUALSIGN: u8 = 1;

// '$'
pub const TOKEN_DOLLARSIGN: u8 = 2;

// '>'
pub const TOKEN_GREATER: u8 = 3;

// '<'
pub const TOKEN_LESS: u8 = 4;

// '('
pub const TOKEN_LPARENT: u8 = 5;

// ')'
pub const TOKEN_RPARENT: u8 = 6;

// '-'
pub const TOKEN_MINUS: u8 = 7;

// ','
pub const TOKEN_COMMA: u8 = 8;

// ';'
pub const TOKEN_SEMICOLON: u8 = 9;

// '{'
pub const TOKEN_LBRACKET: u8 = 10;

// '}'
pub const TOKEN_RBRACKET: u8 = 11;

// '"'
pub const TOKEN_DOUBLEQUOTE: u8 = 12;

// '''
pub const TOKEN_SINGLEQUOTE: u8 = 13;

// '`'
pub const TOKEN_QUOTESPECIAL: u8 = 14;

// used for setting vars
// '@'
pub const TOKEN_ATSIGN: u8 = 15;

// double character symbols

// "=>"
pub const TOKEN_BIGARROWRIGHT: u8 = 50;

// "<="
// Also greater than and equal to
pub const TOKEN_BIGARROWLEFT: u8 = 51;

// Used for incrementing for loops.
// "->"
pub const TOKEN_THINARROWRIGHT: u8 = 52;

// Used for decrementing for loops.
// "<-"
pub const TOKEN_THINARROWLEFT: u8 = 53;

//  "=="
pub const TOKEN_CMPEQUAL: u8 = 54;

pub const TOKEN_LESSEQUAL: u8 = 55;

//  "()"
pub const TOKEN_EMPTYPARENT: u8 = 56;

// "''"
pub const TOKEN_DOUBLESINGLEQUOTE: u8 = 57;


// soyscript keywords

// init
pub const KEYWORD_INIT: u8 = 100;

// if
pub const KEYWORD_IF: u8 = 101;

// func
pub const KEYWORD_FUNC: u8 = 102;

// used to call a function
pub const KEYWORD_CALL: u8 = 103;

// used to repeat the inner call with the same thing
// EXAMPLE:
// ~ original
// repeat 3 {
// call hello()
// }
// ~ expanded
// call hello()
// call hello()
// call hello()
pub const KEYWORD_REPEAT: u8 = 104;

// used to call a function
pub const KEYWORD_FOR: u8 = 105;

// used to call a function
pub const KEYWORD_JSFUNC: u8 = 106;

// Gets the token id of the character given a char.
// Token ids can be found on the top of the file.
// Returns 0 if the char does not have a token id.
// EXAMPLE:
// assert!(get_symbol_id('=') == 1);
pub fn get_symbol_id(cc: char) -> u8 {
    ((cc == '=') as u8 * TOKEN_EQUALSIGN)
        + ((cc == '$') as u8 * TOKEN_DOLLARSIGN)
        + ((cc == '<') as u8 * TOKEN_GREATER)
        + ((cc == '>') as u8 * TOKEN_LESS)
        + ((cc == '(') as u8 * TOKEN_LPARENT)
        + ((cc == ')') as u8 * TOKEN_RPARENT)
        + ((cc == '-') as u8 * TOKEN_MINUS)
        + ((cc == ',') as u8 * TOKEN_COMMA)
        + ((cc == ';') as u8 * TOKEN_SEMICOLON)
        + ((cc == '{') as u8 * TOKEN_LBRACKET)
        + ((cc == '}') as u8 * TOKEN_RBRACKET)
        + ((cc == '"') as u8 * TOKEN_DOUBLEQUOTE)
        + ((cc == '\'') as u8 * TOKEN_SINGLEQUOTE)
        + ((cc == '`') as u8 * TOKEN_QUOTESPECIAL)
        + ((cc == '@') as u8 * TOKEN_ATSIGN)
}
// Gests the double token id given the previous char id (pc) and current char (cc).
// The pc is a u8 because it saves on recomputation.
pub fn get_double_symbol_id(pc: u8, cc: char) -> u8 {
    // =>
    ((pc == TOKEN_EQUALSIGN && cc == '>' ) as u8 * TOKEN_BIGARROWRIGHT ) +
    // <=
    ((pc == TOKEN_LESS && cc == '=' ) as u8 * TOKEN_BIGARROWLEFT ) +
    // ->
    ((pc == TOKEN_MINUS && cc == '>' ) as u8 * TOKEN_THINARROWRIGHT ) +
    // <-
    ((pc == TOKEN_LESS && cc == '-' ) as u8 * TOKEN_THINARROWLEFT ) +
    // ()
    ((pc == TOKEN_LPARENT && cc == ')' ) as u8 * TOKEN_EMPTYPARENT ) +

    ((pc == TOKEN_EQUALSIGN && cc == '=' ) as u8 * TOKEN_CMPEQUAL ) +

    ((pc == TOKEN_SINGLEQUOTE && cc == '\'' ) as u8 * TOKEN_DOUBLESINGLEQUOTE )
}
// jsfunc a() ``
pub fn is_white_space(c: char) -> bool {
    return (c == ' ') | (c == '\n') | (c == '\t');
}
pub fn get_keyword_id(var: &str) -> u8 {
    ((var == "init") as u8 * KEYWORD_INIT)
        + ((var == "if") as u8 * KEYWORD_IF)
        + ((var == "func") as u8 * KEYWORD_FUNC)
        + ((var == "call") as u8 * KEYWORD_CALL)
        + ((var == "repeat") as u8 * KEYWORD_REPEAT)
        + ((var == "for") as u8 * KEYWORD_FOR)
        + ((var == "jsfunc") as u8 * KEYWORD_JSFUNC)
}
