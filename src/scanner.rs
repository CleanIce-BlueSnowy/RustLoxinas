//! 词法解析扫描模块

use std::num;
use std::rc::Rc;
use crate::position::Position;
use crate::tokens::{Token, TokenFloat, TokenInteger, TokenKeyword, TokenOperator, TokenType};
use crate::tokens::TokenParen::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenFloat::*;
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenInteger::*;

/// 词法扫描解析器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TokenScanner<'a> {
    /// 源代码
    source: &'a str,
    /// 所有令牌
    tokens: Vec<Rc<Token>>,
    /// 某个令牌的起始位置
    start: usize,
    /// 下一个要扫描的字符位置
    current: usize,
    /// 行数
    line: usize,
    /// 此行之前已经扫描的字符数量，用于定位行内位置
    scanned_chars: usize,
    /// 存储字符
    chars: Vec<char>,
}

/// 词法错误
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LexicalError {
    pub pos: Position,
    pub message: String,
}

impl LexicalError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { pos: pos.clone(), message }
    }
}

impl<'a> TokenScanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, tokens: Vec::new(), start: 0, current: 0, line: 1, scanned_chars: 0, chars: Vec::new() }
    }

    /// 扫描所有令牌
    pub fn scan_tokens(&mut self) -> Result<(), LexicalError> {
        self.chars = self.source.chars().collect();  // 获取字符

        while !self.is_at_end() {
            self.start = self.current;  // 设置新的令牌开头
            self.scan_token()?;
        }

        // 结尾令牌，方便语法分析
        self.tokens.push(Rc::new(Token::new(TokenType::EOF, 0, self.chars.len(), self.chars.len())));
        return Ok(());
    }

    /** 获取令牌和源码字符串
    # 警告
    这个函数将会移动 `self`！
     */
    pub fn get_tokens(self) -> Vec<Rc<Token>> {
        self.tokens
    }

    /// 扫描单个令牌
    fn scan_token(&mut self) -> Result<(), LexicalError> {
        let ch = self.advance();  // 消耗字符
        match ch {
            '\n' => {
                self.line += 1;
                self.scanned_chars = self.current;  // 更新已扫描的字符
            }
            '(' => self.add_token(TokenType::Paren(LeftParen)),
            ')' => self.add_token(TokenType::Paren(RightParen)),
            '[' => self.add_token(TokenType::Paren(LeftSqrBracket)),
            ']' => self.add_token(TokenType::Paren(RightSqrBracket)),
            '{' => self.add_token(TokenType::Paren(LeftBrace)),
            '}' => self.add_token(TokenType::Paren(RightBrace)),
            '+' => self.add_token(TokenType::Operator(Plus)),
            '-' => self.add_token(TokenType::Operator(Minus)),
            '/' => {
                if self.can_match('/') {
                    // 忽略注释后的内容
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Operator(Slash));
                }
            }
            '*' => {
                if self.can_match('*') {
                    self.add_token(TokenType::Operator(Power));  // 幂运算符
                } else {
                    self.add_token(TokenType::Operator(Star));
                }
            }
            '\\' => self.add_token(TokenType::Operator(Backslash)),
            '&' => self.add_token(TokenType::Operator(TokenOperator::And)),
            '|' => self.add_token(TokenType::Operator(Pipe)),
            '~' => self.add_token(TokenType::Operator(Tilde)),
            ':' => {
                if self.can_match(':') {
                    self.add_token(TokenType::Operator(DoubleColon));
                } else {
                    self.add_token(TokenType::Operator(Colon));
                }
            }
            ';' => self.add_token(TokenType::Operator(Semicolon)),
            '=' => {
                if self.can_match('=') {
                    self.add_token(TokenType::Operator(EqualEqual));
                } else {
                    self.add_token(TokenType::Operator(Equal));
                }
            }
            '<' => {
                if self.can_match('=') {
                    self.add_token(TokenType::Operator(LessEqual));
                } else {
                    self.add_token(TokenType::Operator(Less));
                }
            }
            '>' => {
                if self.can_match('=') {
                    self.add_token(TokenType::Operator(GreaterEqual));
                } else {
                    self.add_token(TokenType::Operator(Greater));
                }
            }
            '!' => {
                if self.can_match('=') {
                    self.add_token(TokenType::Operator(NotEqual));
                } else {
                    self.add_token(TokenType::Operator(Bang));
                }
            }
            '^' => self.add_token(TokenType::Operator(Caret)),
            '%' => self.add_token(TokenType::Operator(Mod)),
            '"' => self.scan_string(false)?,
            '\'' => self.scan_char()?,
            _ if self.is_identifier_char(ch, true) => {  // 标识符、关键字、字符串前缀
                while self.is_identifier_char(self.peek(), false) {
                    self.advance();
                }

                if self.peek() == '"' {  // 识别为字符串前缀
                    self.advance();  // 消耗引号
                    self.scan_string(true)?;
                } else {
                    let word = self.get_whole_word();  // 完整令牌
                    let res = self.check_keyword(&word);  // 检查关键字
                    match res {
                        Some(token) => self.add_token(TokenType::Keyword(token)),
                        None => self.add_token(TokenType::Identifier(word)),
                    }
                }
            }
            _ if ch.is_numeric() => {
                self.scan_number()?;
            }
            _ if ch.is_whitespace() => (),
            other => {
                self.throw_error(&format!("Invalid character: `{other}`"))?;
            }
        }
        return Ok(());
    }

    /// 添加令牌并自动填写位置信息
    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Rc::new(Token::new(token_type, self.line, self.start - self.scanned_chars, self.current - self.scanned_chars)));
    }

    /// 消耗字符
    fn advance(&mut self) -> char {
        let ch = self.chars[self.current];
        self.current += 1;
        return ch;
    }

    /// 是否为合法的标识符字符，开始字符不能为数字，支持下划线
    fn is_identifier_char(&self, ch: char, at_start: bool) -> bool {
        if at_start {
            ch.is_alphabetic() || ch == '_'
        } else {
            ch.is_alphanumeric() || ch == '_'
        }
    }

    /// 是否在字符串尾
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    /// 是否可以匹配。若可以，则消耗字符
    fn can_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    /// 返回下一个字符。支持末尾检查
    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.chars[self.current] }
    }

    /** 返回错误，包含错误类型、行数、位置、错误信息、行内容、位置提示
    
    自动调用 `throw_error_at()` 并填写位置信息
     */
    fn throw_error(&self, msg: &str) -> Result<(), LexicalError> {
        self.throw_error_at(msg, self.start, self.current)
    }

    /** 返回错误，包含错误类型、行数、位置、错误信息、行内容、位置提示
    
    需要手动传入位置信息
     */
    fn throw_error_at(&self, msg: &str, start: usize, end: usize) -> Result<(), LexicalError> {
        Err(LexicalError::new(
            &Position::new(self.line, start, self.line, end),
            msg.to_string(),
        ))
    }

    /// 获取整个令牌
    fn get_whole_word(&mut self) -> String {
        let mut res = String::new();
        for &ch in &self.chars[self.start..self.current] {
            res.push(ch);
        }
        return res;
    }

    /// 检查是否为关键字
    fn check_keyword(&self, word: &str) -> Option<TokenKeyword> {
        match word {
            "if" => Some(If),
            "else" => Some(Else),
            "elif" => Some(Elif),
            "for" => Some(For),
            "while" => Some(While),
            "func" => Some(Func),
            "return" => Some(Return),
            "and" => Some(TokenKeyword::And),
            "or" => Some(Or),
            "not" => Some(Not),
            "let" => Some(Let),
            "true" => Some(True),
            "false" => Some(False),
            "as" => Some(As),
            _ => None,
        }
    }

    /// 扫描数字
    fn scan_number(&mut self) -> Result<(), LexicalError> {
        let mut has_dot = false;  // 是否含有 `.`（是否为浮点数）
        let mut ch = self.chars[self.current - 1];  // 获取当前字符

        if ch == '.' {
            has_dot = true;
        }

        ch = self.peek();
        while ch == '.' || ch.is_numeric() {  // 符合条件
            self.advance();  // 消耗
            if ch == '.' {
                if has_dot {  // 多余的点
                    self.throw_error("Invalid number.")?;
                } else {
                    has_dot = true;
                }
            }
            ch = self.peek();  // 下一个
        }

        let end = self.current;  // 标记数字结尾。此后的都是数字标记

        /// 所有数字类型
        enum NumberType {
            /// 避免编译器产生未初始化的错误。实际情况中，不应该是此值
            NoneForDebug,

            Byte,
            SByte,
            Short,
            UShort,
            Int,
            UInt,
            Long,
            ULong,
            ExtInt,
            UExtInt,
            Float,
            Double
        }

        let mut number_type = NumberType::NoneForDebug;

        if ch.is_alphabetic() {  // 确实存在数字标记。注意，`ch` 已经是下一个字符了（循环末尾的 `self.peek()`）
            self.advance();  // 消耗

            if has_dot {  // 浮点数
                match ch {
                    'd' => number_type = NumberType::Double,
                    'f' => number_type = NumberType::Float,
                    _ => self.throw_error(&format!("Unexpected floating number tag `{}`.", ch))?,
                }
            } else {  // 整数
                match ch {
                    'b' => number_type = NumberType::Byte,
                    's' => {
                        if self.can_match('b') {
                            number_type = NumberType::SByte;
                        } else {
                            number_type = NumberType::Short;
                        }
                    }
                    'i' => number_type = NumberType::Int,
                    'l' => number_type = NumberType::Long,
                    'e' => number_type = NumberType::ExtInt,
                    'u' => {
                        match self.peek() {
                            's' => {
                                number_type = NumberType::UShort;
                                self.advance();
                            }
                            'i' => {
                                number_type = NumberType::UInt;
                                self.advance();
                            }
                            'l' => {
                                number_type = NumberType::ULong;
                                self.advance();
                            }
                            'e' => {
                                number_type = NumberType::UExtInt;
                                self.advance();
                            }
                            _ if !self.peek().is_alphanumeric() => number_type = NumberType::UInt,  // 若后面没有更多合法的标记字符
                            _ => self.throw_error_at(&format!("Unexpected integer number tag `{}`.", ch), self.current, self.current + 1)?,
                        }
                    }
                    _ => self.throw_error_at(&format!("Unexpected integer number tag `{}`.", ch), self.current - 1, self.current)?,
                }
            }
        } else {  // 提供默认类型
            number_type = if has_dot { NumberType::Double } else { NumberType::Int };
        }

        if self.peek().is_alphanumeric() {  // 还有更多字符
            self.throw_error_at(&format!("Unexpected character `{}`.", self.peek()), self.current, self.current + 1)?;
        }

        if has_dot {  // 解析浮点数
            let literal = &self.chars[self.start..end];
            let mut to_parse = String::new();

            // 补充前置省略 0
            if literal[0] == '.' {
                to_parse.push('0');
            }

            for &ch in literal {
                to_parse.push(ch);
            }

            // 补充后置省略 0
            if literal[literal.len() - 1] == '.' {
                to_parse.push('0');
            }

            /// 用于解析浮点数并捕获错误
            fn parse_float(to_parse: String, number_type: NumberType) -> Result<TokenFloat, num::ParseFloatError> {
                match number_type {
                    NumberType::Float => Ok(Float(to_parse.parse::<f32>()?)),
                    NumberType::Double => Ok(Double(to_parse.parse::<f64>()?)),
                    NumberType::NoneForDebug => panic!("Logical Error! Checked NoneForDebug."),  // 不应出现的值
                    _ => panic!("Logical Error! Invalid number type."),  // 其他值
                }
            }

            match parse_float(to_parse, number_type) {
                Ok(res) => self.add_token(TokenType::Float(res)),
                Err(err) => panic!("Logical Error! Unexpected error: {}", err),  // 浮点数转换错误只可能是解析器内部问题
            }
        } else {
            let literal = &self.chars[self.start..end];
            let mut to_parse = String::new();

            for &ch in literal {
                to_parse.push(ch);
            }

            /// 用于解析整数并捕获错误
            fn parse_int(to_parse: String, number_type: NumberType) -> Result<TokenInteger, num::ParseIntError> {
                match number_type {
                    NumberType::Byte => Ok(Byte(to_parse.parse::<u8>()?)),
                    NumberType::SByte => Ok(SByte(to_parse.parse::<i8>()?)),
                    NumberType::Short => Ok(Short(to_parse.parse::<i16>()?)),
                    NumberType::UShort => Ok(UShort(to_parse.parse::<u16>()?)),
                    NumberType::Int => Ok(Int(to_parse.parse::<i32>()?)),
                    NumberType::UInt => Ok(UInt(to_parse.parse::<u32>()?)),
                    NumberType::Long => Ok(Long(to_parse.parse::<i64>()?)),
                    NumberType::ULong => Ok(ULong(to_parse.parse::<u64>()?)),
                    NumberType::ExtInt => Ok(ExtInt(to_parse.parse::<i128>()?)),
                    NumberType::UExtInt => Ok(UExtInt(to_parse.parse::<u128>()?)),
                    NumberType::NoneForDebug => panic!("Logical Error! Checked NoneForDebug."),  // 不应出现的值
                    _ => panic!("Logical Error! Invalid number type."),  // 其他值
                }
            }

            match parse_int(to_parse, number_type) {
                Ok(res) => self.add_token(TokenType::Integer(res)),
                Err(err) => {
                    match err.kind() {
                        num::IntErrorKind::PosOverflow => self.throw_error("Numer is too large.")?,  // 整型溢出应该是用户代码的问题，因此返回词法错误
                        _ => panic!("Logical Error! Unexpected error: {err}."),  // 其余转换错误是解析器的问题
                    }
                }
            }
        }
        return Ok(());
    }

    /// 扫描字符串
    fn scan_string(&mut self, has_pref: bool) -> Result<(), LexicalError> {
        let mut raw_string = false;  // 原始字符串

        if has_pref {  // 含有前缀
            let pref = &self.chars[self.start..(self.current - 1)];  // 获取字符串前缀
            for &ch in pref {
                match ch {
                    'r' => raw_string = true,  // 原始字符串
                    _ => self.throw_error(&format!("Invalid string prefix: `{ch}`"))?,
                }
            }
        }

        let mut res = String::new();
        let mut ch = self.peek();
        while ch != '"' && ch != '\n' && ch != '\0' {  // 注意边界处理
            self.advance();  // 消耗
            if !raw_string && ch == '\\' {
                res.push(self.escape_char(self.peek())?);
                self.advance();  // 消耗转义符
            } else {
                res.push(ch);
            }
            ch = self.peek();
        }

        if ch == '"' {
            self.advance();  // 消耗引号
        } else {
            self.throw_error("Unterminated string.")?;  // 未闭合的字符串
        }

        self.add_token(TokenType::String(res));
        return Ok(());
    }

    /// 扫描字符
    fn scan_char(&mut self) -> Result<(), LexicalError> {
        if self.can_match('\'') {  // 空字符
            self.throw_error("Empty character.")?;
        }

        let mut ch = self.advance();  // 获取字符
        if ch == '\\' {
            let esc = self.advance();
            ch = self.escape_char(esc)?;  // 转义
        }

        return if self.can_match('\'') {  // 结束引号
            self.add_token(TokenType::Char(ch));
            Ok(())
        } else {
            self.throw_error("Unterminated character.")
        }
    }

    /// 转义字符串
    fn escape_char(&self, ch: char) -> Result<char, LexicalError> {
        let mut res = Ok('\0');
        match ch {
            'n' => res = Ok('\n'),
            't' => res = Ok('\t'),
            '0' => res = Ok('\0'),
            'r' => res = Ok('\r'),
            '\\' => res = Ok('\\'),
            '\'' => res = Ok('\''),
            '"' => res = Ok('"'),
            _ => self.throw_error_at(&format!("Unknown escape character: `\\{ch}`"), self.current, self.current + 1)?,
        }
        return res;
    }
}
