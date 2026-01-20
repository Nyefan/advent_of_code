use AOCError::ParseError;
use ParseErrors::*;

#[derive(Debug)]
pub enum AOCError {
    ParseError(ParseErrors),
}
#[derive(Debug)]
pub enum ParseErrors {
    InputFormatError(),
    ParseIntError(std::num::ParseIntError),
    NomParseError(),
}
impl From<std::num::ParseIntError> for AOCError {
    fn from(e: std::num::ParseIntError) -> Self {
        ParseError(ParseIntError(e))
    }
}
impl<E> From<nom::Err<E>> for AOCError {
    fn from(_: nom::Err<E>) -> Self {
        ParseError(NomParseError())
    }
}
