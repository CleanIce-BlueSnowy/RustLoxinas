//! 语法解析——语句解析模块

use crate::{expr_get_pos, parser_can_match, parser_check, parser_consume};
use crate::errors::error_types::{SyntaxError, SyntaxResult};
use crate::expr::Expr;
use crate::parser::Parser;
use crate::position::Position;
use crate::stmt::{Stmt, StmtExpr, StmtLet};
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenType::*;

impl Parser {
    /// 解析单个语句
    pub fn statement(&mut self) -> SyntaxResult<Stmt> {
        if parser_can_match!(self, Keyword(Let)) {
            self.let_stmt()
        } else {
            self.expr_stmt()
        }
    }
    
    /// 表达式语句
    fn expr_stmt(&mut self) -> SyntaxResult<Stmt> {
        let expr = self.expression()?;
        let expr_pos = expr_get_pos!(&expr);
        parser_consume!(
            self, 
            Operator(Semicolon), 
            &Position::new(expr_pos.end_line, expr_pos.end_idx, expr_pos.end_line, expr_pos.end_idx + 1),
            "Expect ';' after a statement.".to_string()
        )?;
        return Ok(Stmt::Expr(StmtExpr {
            expression: Box::new(expr),
        }));
    }
    
    /// `let` 语句
    fn let_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_let = self.previous();
        let let_pos = Position::new(keyword_let.line, keyword_let.start, keyword_let.line, keyword_let.end);
        let token = self.peek();
        let (name, name_pos) = if let Identifier(temp) = &token.token_type {
            self.advance();
            (temp, Position::new(token.line, token.start, token.line, token.end))
        } else {
            let token = self.peek();
            return Err(SyntaxError::new(
                &Position::new(token.line, token.start, token.line, token.end),
                "Expect variable name.".to_string()
            ));
        };
        let var_type = if parser_can_match!(self, Operator(Colon)) {
            Some(self.parse_type_tag()?)
        } else {
            None
        };
        let init = if parser_can_match!(self, Operator(Equal)) {
            Some(self.expression()?)
        } else {
            None
        };
        let end_pos = if let Some(init) = &init {
            let pos = expr_get_pos!(init);
            Position::new(pos.end_line, pos.end_idx, pos.end_line, pos.end_idx + 1)
        } else {
            Position::new(name_pos.end_line, name_pos.end_idx, name_pos.end_line, name_pos.end_idx + 1)
        };
        parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string())?;
        return Ok(Stmt::Let(StmtLet {
            let_pos,
            name_pos,
            name: name.clone(),
            var_type,
            init: if let Some(init) = init { Some(Box::new(init)) } else { None },
        }));
    }
}
