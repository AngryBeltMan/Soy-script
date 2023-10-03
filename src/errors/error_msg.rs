#[derive(Clone, Debug)]
pub enum LexerError {

}

#[derive(Clone, Debug)]
pub enum CompilerError {
    ExpectedIdent,
    ExpectedSymbol,
    ExpectedToken
}
pub trait UnwrapMsg {
    type ReturnItem;
    fn unwrap_or_error(self, error: CompilerError, msg: &str) -> Self::ReturnItem;
}
impl <T, E>UnwrapMsg for Result<T, E> {
    type ReturnItem = T;
    fn unwrap_or_error(self, error: CompilerError, msg: &str) -> Self::ReturnItem {
        match self {
            Ok(o) => o,
            Err(_) => {
                panic!("ERROR {error:?}:\n{msg}");
            }
        }
    }

}

impl <T>UnwrapMsg for Option<T> {
    type ReturnItem = T;
    fn unwrap_or_error(self, error: CompilerError, msg: &str) -> Self::ReturnItem {
        match self {
            Some(o) => o,
            None => {
                panic!("ERROR {error:?}:\n{msg}");
            }
        }
    }

}
