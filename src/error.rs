use derive_more::Display;

use crate::tokenizer::TokenType;

/// A error occurring during URL pattern construction, or matching.
#[derive(Display)]
pub enum Error {
  #[display(fmt = "a relative input without a base URL is not valid")]
  BaseUrlRequired,

  #[display(
    fmt = "specifying both an init object, and a separate base URL is not valid"
  )]
  BaseUrlWithInit,

  #[display(fmt = "tokenizer error: {_0} (at char {_1})")]
  Tokenizer(TokenizerError, usize),

  #[display(fmt = "parser error: {_0}")]
  Parser(ParserError),

  Url(url::ParseError),

  #[display(fmt = "regexp error")]
  RegExp(()),
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self, f)
  }
}

#[derive(Debug, Display)]
pub enum TokenizerError {
  #[display(fmt = "incomplete escape code")]
  IncompleteEscapeCode,
  #[display(fmt = "invalid name; must be at least length 1")]
  InvalidName,
  #[display(fmt = "invalid regex: {_0}")]
  InvalidRegex(&'static str),
}

#[derive(Debug, Display)]
pub enum ParserError {
  #[display(fmt = "expected token {_0}, found '{_2}' of type {_1}")]
  ExpectedToken(TokenType, TokenType, String),

  #[display(fmt = "pattern contains duplicate name {_0}")]
  DuplicateName(String),
}
