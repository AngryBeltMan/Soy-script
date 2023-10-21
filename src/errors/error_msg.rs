#[derive(Clone, Debug)]
pub enum LexerError {

}

#[derive(Clone, Debug)]
pub enum CompilerError {
    ExpectedIdent,
    ExpectedSymbol,
    ExpectedUnsignedInteger,
    ExpectedToken,
    UnknownInlineFunc,
    IncorrectAmountOfArgs,
    UnclosedFunction,
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

#[inline]
pub fn compiler_assert(cmp: bool,error: CompilerError, msg: &str) {
    assert!(cmp, "ERROR {error:?}:\n{msg}");
}

#[inline]
pub fn compiler_panic(error: CompilerError, msg: &str) -> ! {
    panic!("ERROR {error:?}:\n{msg}");
}
