use crate::location::Location;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TokenType {
    EOF,
    OpePlus,
    OpeSub,
    OpeStar,
    OpeSlash,
    OpeEqual,
    Identifier(String),
}

impl Token {
    pub fn make_eof() -> Self {
        Self {
            token_type: TokenType::EOF,
            location: Location::create(0, 0, 0, 0),
        }
    }
}
