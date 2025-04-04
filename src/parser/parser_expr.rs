//! 语法解析——表达式解析模块

use crate::{expr_get_pos, parser_can_match, parser_consume};
use crate::data::{Data, DataFloat, DataInteger};
use crate::errors::error_types::{SyntaxError, SyntaxResult};
use crate::expr::{Expr, ExprAs, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary, ExprVariable};
use crate::parser::Parser;
use crate::parser_check;
use crate::position::Position;
use crate::tokens::{TokenFloat, TokenOperator, TokenParen, TokenType};
use crate::tokens::TokenFloat::*;
use crate::tokens::TokenInteger::*;
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenType::*;

impl Parser {
    /// 解析表达式
    pub fn parse_expression(&mut self) -> SyntaxResult<Expr> {
        self.equality()
    }

    /// 判等表达式
    fn equality(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.comparison()?;
        while parser_can_match!(self, Operator(NotEqual | EqualEqual)) {
            let operator = self.previous();
            let right = self.comparison()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }
    
    /// 比较表达式
    fn comparison(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.binary_shift()?;
        while parser_can_match!(self, Operator(Greater | GreaterEqual | Less | LessEqual)) {
            let operator = self.previous();
            let right = self.binary_shift()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }
    
    /// 位移表达式
    fn binary_shift(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.binary_bit()?;
        while parser_can_match!(self, Keyword(Shl | Shr)) {
            let operator = self.previous();
            let right = self.binary_bit()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }
    
    /// 二元位操作
    fn binary_bit(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.term()?;
        while parser_can_match!(self, Operator(Pipe | TokenOperator::And | Caret)) {
            let operator = self.previous();
            let right = self.term()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }

    /// 加减表达式
    fn term(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.factor()?;
        while parser_can_match!(self, Operator(Plus | Minus)) {
            let operator = self.previous();
            let right = self.factor()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }

    /// 乘除表达式
    fn factor(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.power()?;
        while parser_can_match!(self, Operator(Star | Slash | Mod)) {
            let operator = self.previous();
            let right = self.power()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }

    /// 幂表达式
    fn power(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.unary()?;
        while parser_can_match!(self, Operator(Power)) {
            let operator = self.previous();
            let right = self.unary()?;
            let pos_left = expr_get_pos!(&expr);
            let pos_right = expr_get_pos!(&right);
            expr = Expr::Binary(Box::new(ExprBinary {
                pos: Position::new(pos_left.start_line, pos_left.start_idx, pos_right.end_line, pos_right.end_idx),
                left: expr,
                operator,
                right,
            }));
        }
        return Ok(expr);
    }

    /// 单元运算符表达式
    fn unary(&mut self) -> SyntaxResult<Expr> {
        if parser_can_match!(self, Operator(Minus | Tilde) | Keyword(Not)) {
            let operator = self.previous();
            let right = self.unary()?;
            let pos = expr_get_pos!(&right);
            Ok(Expr::Unary(Box::new(ExprUnary {
                pos: Position::new(operator.line, operator.start, pos.end_line, pos.end_idx),
                operator,
                right,
            })))
        } else {
            self.as_cast()
        }
    }

    /// 类型转换表达式
    fn as_cast(&mut self) -> SyntaxResult<Expr> {
        let mut expr = self.primary()?;
        while parser_can_match!(self, Keyword(As)) {
            let tag = self.parse_type_tag()?;
            let pos = expr_get_pos!(&expr);
            expr = Expr::As(Box::new(ExprAs {
                pos: Position::new(pos.start_line, pos.start_idx, tag.pos.end_line, tag.pos.end_idx),
                expression: expr,
                target: tag,
            }));
        }
        return Ok(expr);
    }

    /// 基本表达式
    fn primary(&mut self) -> SyntaxResult<Expr> {
        let token = self.peek();
        let pos = Position::new(token.line, token.start, token.line, token.end);
        return if parser_can_match!(self, Keyword(False)) {
            Ok(Expr::Literal(Box::new(ExprLiteral { pos, value: Data::Bool(false) })))
        } else if parser_can_match!(self, Keyword(True)) {
            Ok(Expr::Literal(Box::new(ExprLiteral { pos, value: Data::Bool(true) })))
        } else if parser_can_match!(self, Integer(_)) {
            Ok(Expr::Literal(Box::new(ExprLiteral {
                pos,
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
                                ExtInt(extint) => DataInteger::ExtInt(*extint),
                                UExtInt(uextint) => DataInteger::UExtInt(*uextint),
                            }
                        }
                        _ => panic!("Invalid token"),
                    }
                ),
            })))
        } else if parser_can_match!(self, TokenType::Float(_)) {
            Ok(Expr::Literal(Box::new(ExprLiteral {
                pos,
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
                ),
            })))
        } else if parser_can_match!(self, Char(_)) {
            Ok(Expr::Literal(Box::new(ExprLiteral {
                pos,
                value: Data::Char(
                    match &self.previous().token_type {
                        Char(ch) => *ch,
                        _ => panic!("Invalid token"),
                    }
                ),
            })))
        } else if parser_can_match!(self, String(_)) {
            Ok(Expr::Literal(Box::new(ExprLiteral {
                pos,
                value: Data::String(
                    match &self.previous().token_type {
                        String(str) => str.clone(),
                        _ => panic!("Invalid token"),
                    }
                ),
            })))
        } else if parser_can_match!(self, Paren(TokenParen::LeftParen)) {
            let expr = self.parse_expression()?;
            let end_token = self.peek();
            parser_consume!(
                self, 
                Paren(TokenParen::RightParen),
                &Position::new(pos.start_line, pos.start_idx, end_token.line, end_token.end),
                "Expect ')' after expression.".to_string()
            )?;
            let final_token = self.previous();
            Ok(Expr::Grouping(Box::new(ExprGrouping {
                pos: Position::new(pos.start_line, pos.start_idx, final_token.line, final_token.end),
                expression: expr,
            })))
        } else if let Identifier(name) = &token.token_type {
            self.advance();
            Ok(Expr::Variable(Box::new(ExprVariable {
                pos,
                name: name.clone(),
            })))
        } else {
            Err(SyntaxError { pos, message: "Invalid expression.".to_string() })
        }
    }
}
