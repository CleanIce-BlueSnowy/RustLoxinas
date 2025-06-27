//! 语法解析——语句解析模块

use crate::errors::error_types::{SyntaxError, SyntaxResult};
use crate::expr::Expr;
use crate::parser::Parser;
use crate::position::Position;
use crate::stmt::{Stmt, StmtAssign, StmtBlock, StmtBreak, StmtContinue, StmtEmpty, StmtExpr, StmtFor, StmtFunc, StmtIf, StmtInit, StmtLet, StmtLoop, StmtReturn, StmtWhile};
use crate::tokens::Token;
use crate::tokens::TokenKeyword::*;
use crate::tokens::TokenOperator::*;
use crate::tokens::TokenParen::*;
use crate::tokens::TokenType::*;
use crate::{expr_get_pos, parser_can_match, parser_check, parser_consume, stmt_get_pos};
use std::rc::Rc;

impl Parser {
    /// 解析单个语句
    pub fn parse_statement(&mut self) -> SyntaxResult<Stmt> {
        if parser_can_match!(self, Operator(Semicolon)) {
            self.empty_stmt()
        } else if parser_can_match!(self, Keyword(Let)) {
            self.let_stmt()
        } else if parser_can_match!(self, Keyword(Init)) {
            self.init_stmt()
        } else if parser_can_match!(self, Keyword(If)) {
            self.if_stmt()
        } else if parser_can_match!(self, Keyword(Loop)) {
            self.loop_stmt()
        } else if parser_can_match!(self, Keyword(While)) {
            self.while_stmt()
        } else if parser_can_match!(self, Keyword(For)) {
            self.for_stmt()
        } else if parser_can_match!(self, Keyword(Break)) {
            self.break_stmt()
        } else if parser_can_match!(self, Keyword(Continue)) {
            self.continue_stmt()
        } else if parser_can_match!(self, Keyword(Func)) {
            self.func_stmt()
        } else if parser_can_match!(self, Keyword(Return)) {
            self.return_stmt()
        } else if parser_can_match!(self, Paren(LeftBrace)) {
            self.block_stmt()
        } else {
            self.start_with_expr()
        }
    }

    /// 空语句
    fn empty_stmt(&mut self) -> SyntaxResult<Stmt> {
        Ok(Stmt::Empty(Box::new(StmtEmpty {
            pos: self.get_final_pos(),
        })))
    }

    /// 以表达式开头的语句
    fn start_with_expr(&mut self) -> SyntaxResult<Stmt> {
        let expr = self.parse_expression()?;
        if parser_can_match!(self, Operator(Equal)) {
            self.assign_stmt(expr)
        } else if parser_can_match!(
            self,
            Operator(
                PlusEqual
                    | MinusEqual
                    | StarEqual
                    | SlashEqual
                    | AndEqual
                    | PipeEqual
                    | CaretEqual
                    | ModEqual
            )
        ) {
            self.desugar_self_assign_stmt(expr, self.previous())
        } else {
            self.expr_stmt(expr)
        }
    }

    /// 语法糖脱糖——自赋值运算符
    fn desugar_self_assign_stmt(
        &mut self,
        expr: Expr,
        assign_operator: Rc<Token>,
    ) -> SyntaxResult<Stmt> {
        use crate::expr::{ExprBinary, ExprVariable};
        use crate::tokens::TokenOperator::*;
        use crate::tokens::TokenType::*;

        let (another_expr, left_pos) = if let Expr::Variable(var) = &expr {
            (
                Expr::Variable(Box::new(ExprVariable {
                    pos: var.pos.clone(),
                    name: var.name.clone(),
                })),
                var.pos.clone(),
            )
        } else {
            return Err(SyntaxError::new(
                &expr_get_pos!(expr),
                "Must be a left value.".to_string(),
            ));
        };

        let operator = Rc::new(Token::new(
            Operator(match &assign_operator.token_type {
                Operator(PlusEqual) => Plus,
                Operator(MinusEqual) => Minus,
                Operator(StarEqual) => Star,
                Operator(SlashEqual) => Slash,
                Operator(ModEqual) => Mod,
                Operator(AndEqual) => And,
                Operator(PipeEqual) => Pipe,
                Operator(CaretEqual) => Caret,
                _ => unreachable!(), // start_with_expr() 中已经检查
            }),
            assign_operator.line,
            assign_operator.start,
            assign_operator.end,
        ));

        let right_expr = self.parse_expression()?;
        let final_pos = self.get_final_pos();

        Ok(Stmt::Assign(Box::new(StmtAssign {
            pos: Position::new(
                left_pos.start_line,
                left_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            assign_vars: vec![expr],
            right_expr: Expr::Binary(Box::new(ExprBinary {
                pos: expr_get_pos!(&right_expr),
                left: another_expr,
                operator,
                right: right_expr,
            })),
        })))
    }

    /// 表达式语句
    fn expr_stmt(&mut self, expr: Expr) -> SyntaxResult<Stmt> {
        let expr_pos = expr_get_pos!(&expr);

        if !self.in_for_update {
            parser_consume!(
                self,
                Operator(Semicolon),
                &self.get_next_pos(),
                "Expect ';' after a statement.".to_string()
            )?;
        }

        let final_pos = self.get_final_pos();
        Ok(Stmt::Expr(Box::new(StmtExpr {
            pos: Position::new(
                expr_pos.start_line,
                expr_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            expression: expr,
        })))
    }

    /// `let` 语句
    fn let_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_let = self.previous();
        let let_pos = Position::new(
            keyword_let.line,
            keyword_let.start,
            keyword_let.line,
            keyword_let.end,
        );

        let mut token = self.peek();
        let mut is_ref = false;
        if let Keyword(Ref) = &token.token_type {
            self.advance();
            token = self.peek();
            is_ref = true;
        }

        let (name, name_pos) = if let Identifier(temp) = &token.token_type {
            self.advance();
            (
                temp,
                Position::new(token.line, token.start, token.line, token.end),
            )
        } else {
            let token = self.peek();
            return Err(SyntaxError::new(
                &Position::new(token.line, token.start, token.line, token.end),
                "Expect variable name.".to_string(),
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

        parser_consume!(
            self,
            Operator(Semicolon),
            &self.get_next_pos(),
            "Expect ';' after a statement.".to_string()
        )?;
        let final_pos = self.get_final_pos();

        Ok(Stmt::Let(Box::new(StmtLet {
            pos: Position::new(
                let_pos.start_line,
                let_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            let_pos,
            name_pos,
            name: name.clone(),
            var_type,
            init: if let Some(init) = init {
                Some(init)
            } else {
                None
            },
            is_ref,
        })))
    }

    /// `init` 语句。
    fn init_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_init = self.previous();
        let key_init_pos = Position::new(
            keyword_init.line,
            keyword_init.start,
            keyword_init.line,
            keyword_init.end,
        );

        let name_token = self.advance();
        let name_token_pos = Position::new(
            name_token.line,
            name_token.start,
            name_token.line,
            name_token.end,
        );

        let name = if let Identifier(temp) = &name_token.token_type {
            temp
        } else {
            return Err(SyntaxError::new(
                &name_token_pos,
                "Expect variable name.".to_string(),
            ));
        };
        
        parser_consume!(
            self, 
            Operator(Equal), 
            &self.get_next_pos(),
            "Expect '='.".to_string()
        )?;

        let init = self.parse_expression()?;
        let init_pos = expr_get_pos!(&init);
        let end_pos = Position::new(
            init_pos.end_line,
            init_pos.end_idx,
            init_pos.end_line,
            init_pos.end_idx + 1,
        );
        parser_consume!(
            self,
            Operator(Semicolon),
            &end_pos,
            "Expect ';' after a statement.".to_string()
        )?;
        let final_pos = self.get_final_pos();

        Ok(Stmt::Init(Box::new(StmtInit {
            pos: Position::new(
                key_init_pos.start_line,
                key_init_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            name_pos: name_token_pos,
            name: name.clone(),
            init,
        })))
    }

    /// 条件判断语句
    fn if_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_if = self.previous();
        let if_pos = Position::new(
            keyword_if.line,
            keyword_if.start,
            keyword_if.line,
            keyword_if.end,
        );

        let if_expr = self.parse_expression()?;
        
        parser_consume!(
            self,
            Paren(LeftBrace),
            &self.get_next_pos(),
            "Except '{' after the expression.".to_string()
        )?;

        let if_chunk = self.block_stmt()?;

        if parser_can_match!(self, Keyword(Else)) {
            if parser_can_match!(self, Keyword(If)) {
                let mut else_if_cases = vec![];

                let else_if_stmt = self.if_stmt()?; // 递归解析，但线性拼接
                let mut else_if = if let Stmt::If(temp) = else_if_stmt {
                    temp
                } else {
                    unreachable!()
                };
                else_if_cases.push((else_if.if_branch.0, else_if.if_branch.1));
                else_if_cases.append(&mut else_if.else_if_branch);
                let final_pos = self.get_final_pos();

                Ok(Stmt::If(Box::new(StmtIf {
                    pos: Position::new(
                        if_pos.start_line,
                        if_pos.start_idx,
                        final_pos.end_line,
                        final_pos.end_idx,
                    ),
                    if_branch: (if_expr, if_chunk),
                    else_if_branch: else_if_cases,
                    else_branch: else_if.else_branch,
                })))
            } else {
                parser_consume!(
                    self,
                    Paren(LeftBrace),
                    &self.get_next_pos(),
                    "Expect '{' after 'else'.".to_string()
                )?;
                let else_chunk = self.block_stmt()?;
                let final_pos = self.get_final_pos();

                Ok(Stmt::If(Box::new(StmtIf {
                    pos: Position::new(
                        if_pos.start_line,
                        if_pos.start_idx,
                        final_pos.end_line,
                        final_pos.end_idx,
                    ),
                    if_branch: (if_expr, if_chunk),
                    else_if_branch: vec![],
                    else_branch: Some(else_chunk),
                })))
            }
        } else {
            let final_pos = self.get_final_pos();

            Ok(Stmt::If(Box::new(StmtIf {
                pos: Position::new(
                    if_pos.start_line,
                    if_pos.start_idx,
                    final_pos.end_line,
                    final_pos.end_idx,
                ),
                if_branch: (if_expr, if_chunk),
                else_if_branch: vec![],
                else_branch: None,
            })))
        }
    }

    /// 无限循环语句
    fn loop_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_loop = self.previous();
        let loop_pos = Position::new(
            keyword_loop.line,
            keyword_loop.start,
            keyword_loop.line,
            keyword_loop.end,
        );

        // 处理标记
        let tag = if let Tag(tag_name) = &self.peek().token_type {
            self.advance();
            Some(tag_name.clone())
        } else {
            None
        };
        
        parser_consume!(
            self, 
            Paren(LeftBrace), 
            &self.get_next_pos(), 
            "Expect '{'.".to_string()
        )?;

        let chunk = self.block_stmt()?;
        let final_pos = self.get_final_pos();

        Ok(Stmt::Loop(Box::new(StmtLoop {
            pos: Position::new(
                loop_pos.start_line,
                loop_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            chunk,
            tag,
        })))
    }

    /// 条件循环语句
    fn while_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_while = self.previous();
        let while_pos = Position::new(
            keyword_while.line,
            keyword_while.start,
            keyword_while.line,
            keyword_while.end,
        );

        // 处理标记
        let tag = if let Tag(tag_name) = &self.peek().token_type {
            self.advance();
            Some(tag_name.clone())
        } else {
            None
        };

        let condition = self.parse_expression()?;
        
        parser_consume!(
            self,
            Paren(LeftBrace),
            &self.get_next_pos(),
            "Expect '{' after the condition.".to_string()
        )?;

        let chunk = self.block_stmt()?;
        let final_pos = self.get_final_pos();

        Ok(Stmt::While(Box::new(StmtWhile {
            pos: Position::new(
                while_pos.start_line,
                while_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            condition,
            chunk,
            tag,
        })))
    }

    fn for_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_for = self.previous();
        let for_pos = Position::new(
            keyword_for.line,
            keyword_for.start,
            keyword_for.line,
            keyword_for.end,
        );

        // 处理标记
        let tag = if let Tag(tag_name) = &self.peek().token_type {
            self.advance();
            Some(tag_name.clone())
        } else {
            None
        };

        let init = self.parse_statement()?;
        let condition = self.parse_expression()?;
        
        parser_consume!(
            self,
            Operator(Semicolon),
            &self.get_next_pos(),
            "Expect ';' after the expression.".to_string()
        )?;

        self.in_for_update = true;
        let update = self.parse_statement()?;
        self.in_for_update = false;
        let update_pos = stmt_get_pos!(&update);

        if !matches!(update, Stmt::Expr(_) | Stmt::Assign(_) | Stmt::If(_)) {
            return Err(SyntaxError::new(&update_pos, "Only expression statement, assign statement and if statement can be used as update statement.".to_string()));
        }
        
        parser_consume!(
            self,
            Paren(LeftBrace),
            &self.get_next_pos(),
            "Expect '{' after the update statement.".to_string()
        )?;

        let chunk = self.block_stmt()?;

        let final_pos = self.get_final_pos();

        Ok(Stmt::For(Box::new(StmtFor {
            pos: Position::new(
                for_pos.start_line,
                for_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            init,
            condition,
            update,
            chunk,
            tag,
        })))
    }

    /// 退出循环语句
    fn break_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_break = self.previous();
        let break_pos = Position::new(
            keyword_break.line,
            keyword_break.start,
            keyword_break.line,
            keyword_break.end,
        );

        // 处理标记
        let tag = if let Tag(tag_name) = &self.peek().token_type {
            Some(tag_name.clone())
        } else {
            None
        };

        parser_consume!(
            self,
            Operator(Semicolon),
            &self.get_next_pos(),
            "Expect ';' after 'break'.".to_string()
        )?;

        let final_pos = self.get_final_pos();

        Ok(Stmt::Break(Box::new(StmtBreak {
            pos: Position::new(
                break_pos.start_line,
                break_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            tag,
        })))
    }

    /// 继续循环语句
    fn continue_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_continue = self.previous();
        let continue_pos = Position::new(
            keyword_continue.line,
            keyword_continue.start,
            keyword_continue.line,
            keyword_continue.end,
        );

        // 处理标记
        let tag = if let Tag(tag_name) = &self.peek().token_type {
            Some(tag_name.clone())
        } else {
            None
        };

        parser_consume!(
            self,
            Operator(Semicolon),
            &self.get_next_pos(),
            "Expect ';' after 'continue'.".to_string()
        )?;

        let final_pos = self.get_final_pos();

        Ok(Stmt::Continue(Box::new(StmtContinue {
            pos: Position::new(
                continue_pos.start_line,
                continue_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            tag,
        })))
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
        if !self.in_for_update {
            parser_consume!(
                self,
                Operator(Semicolon),
                &self.get_next_pos(),
                "Expect ';' after a statement.".to_string()
            )?;
        }
        let final_pos = self.get_final_pos();

        Ok(Stmt::Assign(Box::new(StmtAssign {
            pos: Position::new(
                start_pos.start_line,
                start_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            assign_vars: vars,
            right_expr: next_expr,
        })))
    }

    /// 块语句
    fn block_stmt(&mut self) -> SyntaxResult<Stmt> {
        let left_brace = self.previous();
        let left_brace_pos = Position::new(
            left_brace.line,
            left_brace.start,
            left_brace.line,
            left_brace.end,
        );

        let mut statements = vec![];
        while !parser_check!(self, Paren(RightBrace)) {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }
        parser_consume!(
            self,
            Paren(RightBrace),
            &self.get_next_pos(),
            "Unclosed block statement.".to_string()
        )?;
        let final_pos = self.get_final_pos();

        Ok(Stmt::Block(Box::new(StmtBlock {
            pos: Position::new(
                left_brace_pos.start_line,
                left_brace_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            statements,
        })))
    }

    fn func_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_func = self.previous();
        let func_pos = Position::new(
            keyword_func.line,
            keyword_func.start,
            keyword_func.line,
            keyword_func.end,
        );

        let next_token = self.advance();
        let next_token_pos = Position::new(
            next_token.line,
            next_token.start,
            next_token.line,
            next_token.end,
        );
        let func_name = if let Identifier(name) = &next_token.token_type {
            name
        } else {
            return Err(SyntaxError::new(
                &next_token_pos,
                "Expect an identifier.".to_string(),
            ));
        };

        parser_consume!(
            self, 
            Paren(LeftParen), 
            &self.get_next_pos(), 
            "Expect '('.".to_string()
        )?;
        
        let mut parameters = vec![];

        let mut name_token = self.peek();
        while let Identifier(param_name) = &name_token.token_type {
            self.advance();

            parser_consume!(
                self,
                Operator(Colon),
                &self.get_next_pos(),
                "Expect ':' after parameter name.".to_string(),
            )?;

            let param_type = self.parse_type_tag()?;

            parameters.push((param_name.clone(), param_type));
            
            if let Paren(RightParen) = &self.peek().token_type {
                break;
            }
            
            parser_consume!(
                self,
                Operator(Comma),
                &self.get_next_pos(),
                "Except ',' or ')'.".to_string(),
            )?;
            
            name_token = self.peek();
        }
        
        if parser_check!(self, Operator(Comma)) {
            self.advance();
        }
        
        parser_consume!(
            self,
            Paren(RightParen),
            &self.get_next_pos(),
            "Except ')' after parameter list.".to_string(),
        )?;
        
        let return_type = if parser_can_match!(self, Operator(RightArrow)) {
            Some(self.parse_type_tag()?)
        } else {
            None
        };
        
        parser_consume!(
            self,
            Paren(LeftBrace),
            &self.get_next_pos(),
            "Except '{' after function declaration.".to_string(),
        )?;
        
        let chunk = self.block_stmt()?;
        
        let final_pos = self.get_final_pos();
        
        Ok(Stmt::Func(Box::new(StmtFunc {
            pos: Position::new(
                func_pos.start_line,
                func_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            name: func_name.clone(),
            params: parameters,
            return_type,
            body: chunk,
        })))
    }
    
    /// 返回语句
    fn return_stmt(&mut self) -> SyntaxResult<Stmt> {
        let keyword_return = self.previous();
        let return_pos = Position::new(
            keyword_return.line,
            keyword_return.start,
            keyword_return.line,
            keyword_return.end,
        );
        
        let expr = if parser_can_match!(self, Operator(Semicolon)) {
            None
        } else {
            let expr = self.parse_expression()?;
            
            parser_consume!(
                self,
                Operator(Semicolon),
                &self.get_next_pos(),
                "Expect ';' after return statement.".to_string(),
            )?;
            
            Some(expr)
        };
        
        let final_pos = self.get_final_pos();
        
        Ok(Stmt::Return(Box::new(StmtReturn {
            pos: Position::new(
                return_pos.start_line,
                return_pos.start_idx,
                final_pos.end_line,
                final_pos.end_idx,
            ),
            expr,
        })))
    }
}
