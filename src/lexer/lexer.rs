use crate::lexer::tokens::*;

#[derive(Debug, Clone)]
pub struct SettingVars { pub vars: Vec<String> }
#[derive(Debug, Clone)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub setting_vars: SettingVars
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub symbol_id: u8
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    SingleSymbol,
    DoubleSymbol,
    Ident,
    Keyword
}

impl Lexer {
    pub fn parse_str(file: &str) -> Self {
        let mut tokens = vec![];
        let mut prev_symbol = 0;
        let mut ident_string = String::new();
        // a bool to decide whether to ignore white spaces
        // ignore_whitespace will only be true for string and printing
        // as the user would want spaces
        let mut ignore_whitespace = true;
        let mut is_commented = false;
        let mut is_special_quote = false;
        for character in file.chars() {
            // check whether or not to parse based on if it is a comment
            if character == '\n' { is_commented = false; }
            if character == '`' {
                prev_symbol = 0;
                is_special_quote = !is_special_quote;
                // if we encounter this character we won't even include it in the token stream
                continue;
            }
            // the single line comment character
            if character == '~' { is_commented = true; }
            if is_commented { continue; }

            let symbol = get_symbol_id(character);

            // stops parsing the ident string when it encounters a symbol, whitespace.
            if (is_white_space(character) | (symbol != 0)) && ignore_whitespace && !is_special_quote {
                lexer_insert_ident_and_keyword(&mut ident_string, &mut tokens);
            } else { ident_string.push(character); }

            // swaps ignore white space value if there is double quote
            if character == '"' {
                ignore_whitespace = !ignore_whitespace;
                // checks if we have already inserted a double quote and if we have skip
                if !ignore_whitespace && !is_special_quote {
                    ident_string.push('"');
                }
                continue;
            }
            // if it is parsing a special quote we should not insert any tokens
            // if the current token is nothing parse the previous token
            if (symbol != 0) && !is_special_quote {
                set_symbol_id(&mut prev_symbol, symbol,character, &mut tokens);
            } else {
                // special quote will not insert a single symbol token because we set the previous
                // symbol to zero which is not a token symbol id
                check_push_symbol(&mut prev_symbol, &mut tokens);
            }
        }
        check_push_symbol(&mut prev_symbol, &mut tokens);
        lexer_insert_ident_and_keyword(&mut ident_string, &mut tokens);
        Self { tokens, setting_vars: SettingVars { vars: vec![] } }
    }
}
// checks for double symbols
#[inline]
fn set_symbol_id(prev_symbol: &mut u8, symbol: u8, character: char ,tokens: &mut Vec<Token>) {
    if *prev_symbol == 0 {
        *prev_symbol = symbol;
        return;
    }
    let double_token = get_double_symbol_id(*prev_symbol, character);
    // double symbol doesn't exist so skip inserting a token of type double token
    // we begin inserting single symbols because the current character would be skipped messing up
    // parsing the token is skipped because the previous symbol is not reassigned the current symbol
    // and therefore there would be no way for the symbol to be inserted as it is not stored
    // anywhere else
    if double_token == 0 {
        // pushes the previous symbol because it should come before the current symbol
        push_symbol(tokens, *prev_symbol);
        // we don't have to worry about the return type being zero because we already do checks to
        // make sure the character is a valid symbol before calling this function
        push_symbol(tokens, get_symbol_id(character));
        // assign prev_symbol to zero because we already inserted it into the tokens
        *prev_symbol = 0;
        return;
    }
    tokens.push(Token {
        token_type: TokenType::DoubleSymbol,
        text: String::new(),
        symbol_id: double_token
    });
    *prev_symbol = 0;
}
#[inline]
// checks to see if a symbol need to be inserted
fn check_push_symbol(prev_symbol: &mut u8, tokens: &mut Vec<Token>) {
    if *prev_symbol != 0 {
        push_symbol(tokens, *prev_symbol);
        *prev_symbol = 0;
    }
}
#[inline]
fn push_symbol(tokens: &mut Vec<Token>, prev_symbol: u8) {
    tokens.push(Token {
        token_type: TokenType::SingleSymbol,
        text: String::new(),
        symbol_id: prev_symbol
    });
}
// The function, using the user input string, either inserts a keyword (if it is one), an ident
// token, or does nothing.
fn lexer_insert_ident_and_keyword(ident_string: &mut String, tokens: &mut Vec<Token>) {
    if get_keyword_id(ident_string) != 0 {
        tokens.push(Token { token_type: TokenType::Keyword, text: String::new(), symbol_id: get_keyword_id(&ident_string) })
    } else if ident_string.len() > 0 {
        tokens.push(Token {
            token_type: TokenType::Ident,
            text: ident_string.to_string(), symbol_id: 0
        });
    }
    *ident_string = String::new();
}
