/// 令牌
#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    /// 令牌类型
    pub token_type: TokenType,
    /// 令牌所在行
    pub line: usize,
    /// 令牌起始索引
    pub start: usize,
    /// 令牌终止索引
    pub end: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, start: usize, end: usize) -> Self {
        Self { token_type, line, start, end }
    }
}

/// 通用令牌类型
#[derive(Debug)]
pub enum TokenType {
    Paren(TokenParen),
    Operator(TokenOperator),
    Keyword(TokenKeyword),
    Identifier(String),
    Integer(TokenInteger),
    Float(TokenFloat),
    String(String),
    Char(char),
    EOF,
}

/// 括号令牌
#[derive(Debug)]
pub enum TokenParen {
    LeftParen,  // (
    RightParen,  // )
    LeftSqrBracket,  // [
    RightSqrBracket,  // ]
    LeftBrace,  // {
    RightBrace,  // }
}

/// 运算符令牌
#[derive(Debug)]
pub enum TokenOperator {
    Plus,  // +
    Minus,  // -
    Slash,  // /
    Star,  // *
    Power,  // **
    Backslash,  // \
    And,  // &
    Pipe,  // |
    Tilde,  // ~
    Colon,  // :
    Semicolon,  // ;
    Equal,  // =
    EqualEqual,  // ==
    NotEqual,  // !=
    Less,  // <
    Greater,  // >
    LessEqual,  // <=
    GreaterEqual,  // >=
    Bang,  // !
    Caret,  // ^
}

/// 关键字令牌
#[derive(Debug)]
pub enum TokenKeyword {
    If,
    Else,
    Elif,
    For,
    While,
    Func,
    Return,
    And,
    Or,
    Not,
    Let,
    False,
    True,
}

/// 整型令牌
#[derive(Debug)]
pub enum TokenInteger {
    Byte(u8),
    SByte(i8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    ExtInt(i128),
    UExtInt(u128),
}

/// 浮点型令牌
#[derive(Debug)]
pub enum TokenFloat {
    Float(f32),
    Double(f64),
}
