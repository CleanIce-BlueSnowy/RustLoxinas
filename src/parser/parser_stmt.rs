//! 语法解析——语句解析模块

use crate::{expr_get_pos, parser_can_match, parser_check, parser_consume};
use crate::errors::error_types::{SyntaxError, SyntaxResult};
use crate::expr::Expr;
use crate::parser::Parser;
use crate::position::Position;
use crate::stmt::{Stmt, StmtAssign, StmtExpr, StmtInit, StmtLet, StmtPrint};
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenType::*;

impl Parser {
    /// 解析单个语句
    pub fn statement(&mut self) -> SyntaxResult<Stmt> {
        if parser_can_match!(self, Keyword(Let)) {
            self.let_stmt()
        } else if parser_can_match!(self, Keyword(Init)) {
            self.init_stmt()
        } else if parser_can_match!(self, Keyword(Print)) {
            self.print_stmt()
        } else {
            self.start_with_expr()
        }
    }
    
    /// 以表达式开头的语句
    fn start_with_expr(&mut self) -> SyntaxResult<Stmt> {
        let expr = self.parse_expression()?;
        return if parser_can_match!(self, Operator(Equal)) {
            self.assign_stmt(expr)
        } else {
            self.expr_stmt(expr)
        };
    }
    
    /// 表达式语句
    fn expr_stmt(&mut self, expr: Expr) -> SyntaxResult<Stmt> {
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
        let mut token = self.peek();
        let mut is_ref = false;
        if let Keyword(Ref) = &token.token_type {
            self.advance();
            token = self.peek();
            is_ref = true;
        }
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
            Some(self.parse_expression()?)
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
            is_ref,
        }));
    }
    
    /// `init` 语句。
    fn init_stmt(&mut self) -> SyntaxResult<Stmt> {
        let name_token = self.advance();
        let name_token_pos = Position::new(name_token.line, name_token.start, name_token.line, name_token.end);
        
        let name = if let Identifier(temp) = &name_token.token_type {
            temp
        } else {
            return Err(SyntaxError::new(&name_token_pos, "Expect variable name.".to_string()));
        };
        
        let next = self.peek();
        let err_pos = Position::new(next.line, next.start, next.line, next.end);
        
        parser_consume!(self, Operator(Equal), &err_pos, "Expect '='.".to_string())?;
        
        let init = self.parse_expression()?;
        let init_pos = expr_get_pos!(&init);
        let end_pos = Position::new(init_pos.end_line, init_pos.end_idx, init_pos.end_line, init_pos.end_idx + 1);
        parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string())?;
        
        return Ok(Stmt::Init(StmtInit {
            name_pos: name_token_pos,
            name: name.clone(),
            init: Box::new(init),
        }));
    }
    
    /// 赋值语句
    fn assign_stmt(&mut self, first_var: Expr) -> SyntaxResult<Stmt> {
        let mut vars = vec![Box::new(first_var)];
        let mut next_expr = self.parse_expression()?;
        while parser_can_match!(self, Operator(Equal)) {
            vars.push(Box::new(next_expr));
            next_expr = self.parse_expression()?;
        }
        let pos = expr_get_pos!(&next_expr);
        let end_pos = Position::new(pos.end_line, pos.end_idx, pos.end_line, pos.end_idx + 1);
        parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string())?;
        
        return Ok(Stmt::Assign(StmtAssign {
            assign_vars: vars,
            right_expr: Box::new(next_expr),
        }));
    }
    
    /// 临时辅助功能：打印语句
    fn print_stmt(&mut self) -> SyntaxResult<Stmt> {
        let expr = if parser_can_match!(self, Operator(Semicolon)) {
            None
        } else {
            let res = self.parse_expression()?;
            let pos = expr_get_pos!(&res);
            let end_pos = Position::new(pos.end_line, pos.end_idx, pos.end_line, pos.end_idx + 1);
            parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string())?;
            Some(Box::new(res))
        };
        
        return Ok(Stmt::Print(StmtPrint {
            expr,
        }));
    }
}
