//! 语法解析——语句解析模块

use crate::{expr_get_pos, parser_can_match, parser_check, parser_consume, stmt_get_pos};
use crate::errors::error_types::{SyntaxError, SyntaxResult};
use crate::expr::Expr;
use crate::parser::Parser;
use crate::position::Position;
use crate::stmt::{Stmt, StmtAssign, StmtBlock, StmtExpr, StmtInit, StmtLet, StmtPrint};
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenParen::*;
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
        } else if parser_can_match!(self, Paren(LeftBrace)) {
            self.block_stmt()
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
        let end_pos = Position::new(expr_pos.end_line, expr_pos.end_idx, expr_pos.end_line, expr_pos.end_idx + 1);
        parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string()
        )?;
        let final_pos = self.get_final_pos();
        return Ok(Stmt::Expr(StmtExpr {
            pos: Position::new(expr_pos.start_line, expr_pos.start_idx, final_pos.end_line, final_pos.end_idx),
            expression: expr,
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
        let final_pos = self.get_final_pos();
        return Ok(Stmt::Let(StmtLet {
            pos: Position::new(let_pos.start_line, let_pos.start_idx, final_pos.end_line, final_pos.end_idx),
            let_pos,
            name_pos,
            name: name.clone(),
            var_type,
            init: if let Some(init) = init { Some(init) } else { None },
            is_ref,
        }));
    }
    
    /// `init` 语句。
    fn init_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_init = self.previous();
        let key_init_pos = Position::new(keyword_init.line, keyword_init.start, keyword_init.line, keyword_init.end);
        
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
        let final_pos = self.get_final_pos();
        
        return Ok(Stmt::Init(StmtInit {
            pos: Position::new(key_init_pos.start_line, key_init_pos.start_idx, final_pos.end_line, final_pos.end_idx),
            name_pos: name_token_pos,
            name: name.clone(),
            init,
        }));
    }
    
    /// 赋值语句
    fn assign_stmt(&mut self, first_var: Expr) -> SyntaxResult<Stmt> {
        let start_pos = expr_get_pos!(&first_var);
        
        let mut vars = vec![first_var];
        let mut next_expr = self.parse_expression()?;
        while parser_can_match!(self, Operator(Equal)) {
            vars.push(next_expr);
            next_expr = self.parse_expression()?;
        }
        let pos = expr_get_pos!(&next_expr);
        let end_pos = Position::new(pos.end_line, pos.end_idx, pos.end_line, pos.end_idx + 1);
        parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string())?;
        let final_pos = self.get_final_pos();
        
        return Ok(Stmt::Assign(StmtAssign {
            pos: Position::new(start_pos.start_line, start_pos.start_idx, final_pos.end_line, final_pos.end_idx),
            assign_vars: vars,
            right_expr: next_expr,
        }));
    }

    /// 块语句
    fn block_stmt(&mut self) -> SyntaxResult<Stmt> {
        let left_brace = self.previous();
        let left_brace_pos = Position::new(left_brace.line, left_brace.start, left_brace.line, left_brace.end);
        
        let mut statements = vec![];
        let mut end_pos = Position::new(left_brace_pos.end_line, left_brace_pos.end_idx, left_brace_pos.end_line, left_brace_pos.end_idx + 1);
        while !parser_check!(self, Paren(RightBrace)) {
            let statement = self.statement()?;
            let stmt_pos = stmt_get_pos!(&statement);
            end_pos = Position::new(stmt_pos.end_line, stmt_pos.end_idx, stmt_pos.end_line, stmt_pos.end_idx + 1);
            statements.push(statement);
        }
        parser_consume!(self, Paren(RightBrace), &end_pos, "Unclosed block statement.".to_string())?;
        let final_pos = self.get_final_pos();
        
        return Ok(Stmt::Block(StmtBlock {
            pos: Position::new(left_brace_pos.start_line, left_brace_pos.start_idx, final_pos.end_line, final_pos.end_idx),
            statements,
        }));
    }
    
    /// 临时辅助功能：打印语句
    fn print_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_print = self.previous();
        let print_pos = Position::new(keyword_print.line, keyword_print.start, keyword_print.line, keyword_print.end);
        
        let expr = if parser_can_match!(self, Operator(Semicolon)) {
            None
        } else {
            let res = self.parse_expression()?;
            let pos = expr_get_pos!(&res);
            let end_pos = Position::new(pos.end_line, pos.end_idx, pos.end_line, pos.end_idx + 1);
            parser_consume!(self, Operator(Semicolon), &end_pos, "Expect ';' after a statement.".to_string())?;
            Some(res)
        };
        let final_pos = self.get_final_pos();
        
        return Ok(Stmt::Print(StmtPrint {
            pos: Position::new(print_pos.start_line, print_pos.start_idx, final_pos.end_line, final_pos.end_idx),
            expr,
        }));
    }
}
