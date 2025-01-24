use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::Expr;
use crate::parser::{Parser, SyntaxError};
use crate::parser_can_match;
use crate::parser_check;
use crate::tokens::TokenType::*;
use crate::tokens::TokenInteger::*;
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenFloat::*;
use crate::tokens::{TokenFloat, TokenOperator, TokenType};

impl Parser {
    /// 解析表达式
    pub fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.equality()
    }

    /// 判等表达式
    fn equality(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.comparison()?;
        while parser_can_match!(self, Operator(NotEqual | EqualEqual)) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        return Ok(expr);
    }
    
    /// 比较表达式
    fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.term()?;
        while parser_can_match!(self, Operator(Greater | GreaterEqual | Less | LessEqual)) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        return Ok(expr);
    }

    /// 加减表达式
    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.factor()?;
        while parser_can_match!(self, Operator(Plus | Minus)) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        return Ok(expr);
    }

    /// 乘除表达式
    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.power()?;
        while parser_can_match!(self, Operator(Star | Slash)) {
            let operator = self.previous();
            let right = self.power()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        return Ok(expr);
    }

    /// 幂表达式
    fn power(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.unary()?;
        while parser_can_match!(self, Operator(Power)) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        return Ok(expr);
    }

    /// 单元运算符表达式
    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        if parser_can_match!(self, Operator(Minus | Tilde | TokenOperator::And | Pipe | Caret) | Keyword(Not)) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary { operator, right: Box::new(right) })
        } else {
            self.primary()
        }
    }

    /// 基本表达式
    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        if parser_can_match!(self, Keyword(False)) {
            Ok(Expr::Literal { value: Data::Bool(false) })
        } else if parser_can_match!(self, Keyword(True)) {
            Ok(Expr::Literal { value: Data::Bool(true) })
        } else if parser_can_match!(self, Integer(_)) {
            Ok(Expr::Literal {
                value: Data::Integer(
                    match &self.previous().token_type {
                        Integer(integer) => {
                            match integer {
                                Byte(byte) => DataInteger::Byte(*byte),
                                SByte(sbyte) => DataInteger::SByte(*sbyte),
                                Short(short) => DataInteger::Short(*short),
                                UShort(ushort) => DataInteger::UShort(*ushort),
                                Int(int) => DataInteger::Int(*int),
                                UInt(uint) => DataInteger::UInt(*uint),
                                Long(long) => DataInteger::Long(*long),
                                ULong(ulong) => DataInteger::ULong(*ulong),
                                ExtInt(eint) => DataInteger::ExtInt(*eint),
                                UExtInt(ueint) => DataInteger::UExtInt(*ueint),
                            }
                        }
                        _ => panic!("Invalid token"),
                    }
                )
            })
        } else if parser_can_match!(self, TokenType::Float(_)) {
            Ok(Expr::Literal {
                value: Data::Float(
                    match &self.previous().token_type {
                        TokenType::Float(float) => {
                            match float {
                                TokenFloat::Float(float) => DataFloat::Float(*float),
                                Double(double) => DataFloat::Double(*double),
                            }
                        }
                        _ => panic!("Invalid token"),
                    }
                )
            })
        } else if parser_can_match!(self, Char(_)) {
            Ok(Expr::Literal {
                value: Data::Char(
                    match &self.previous().token_type {
                        Char(ch) => *ch,
                        _ => panic!("Invalid token"),
                    }
                )
            })
        } else if parser_can_match!(self, String(_)) {
            Ok(Expr::Literal {
                value: Data::String(
                    match &self.previous().token_type {
                        String(str) => str.clone(),
                        _ => panic!("Invalid token"),
                    }
                )
            })
        } else {
            Err(SyntaxError { token: self.peek(), message: "Invalid expression.".to_string() })
        }
    }
}
