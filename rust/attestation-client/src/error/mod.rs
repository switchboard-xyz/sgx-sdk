#[derive(Clone, Debug, PartialEq)]
pub enum Err {
    Generic,
    SgxError,
    SgxWriteError,
    AnjunaError(String),
    AnchorParse,
    TxFailure,
    NetworkErr,
    InvalidQuoteError,
    TxCompileErr,
    EnvVariableMissing,
}
