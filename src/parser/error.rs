use crate::token::TokenKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected {0:?}, but found {1:?}")]
    InvalidTokenFound(Vec<TokenKind>, TokenKind),

    #[error("Failed to read token")]
    FailedToReadToken,

    #[error("No such expression that start with {0:?}")]
    NoSuchExpressionStartWith(TokenKind),
}
