/// 令牌
#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    /// 令牌类型
    token_type: TokenType,
    /// 令牌所在行
    line: usize,
    /// 令牌起始索引
    start_idx: usize,
    /// 令牌终止索引
    end_idx: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, start_idx: usize, end_idx: usize) -> Self {
        Self{ token_type, line, start_idx, end_idx }
    }
}

/// 通用令牌类型
#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Paren(TokenParen),
    Operator(TokenOperator),
    Keyword(TokenKeyword),
    Identifier(String),
    Integer(TokenInteger),
    Float(TokenFloat),
    String(String),
}

/// 括号令牌
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    BangEqual,  // !=
}

/// 关键字令牌
#[allow(dead_code)]
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
}

/// 整形令牌
#[allow(dead_code)]
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
#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenFloat {
    Float(f32),
    Double(f64),
}
