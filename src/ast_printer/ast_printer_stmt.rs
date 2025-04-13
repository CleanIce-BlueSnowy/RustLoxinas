//! 语法树打印——语句打印模块

use indexmap::indexmap;

use crate::ast_printer::{AstPrinter, TreeChild};
use crate::stmt::{Stmt, StmtAssign, StmtBlock, StmtBreak, StmtExpr, StmtIf, StmtInit, StmtLet, StmtPrint, StmtVisitor, StmtWhile};

#[cfg(debug_assertions)]
impl StmtVisitor<String> for AstPrinter {
    fn visit_expr_stmt(&mut self, this: *const Stmt, stmt: &StmtExpr) -> String {
        let children = indexmap! {
            "expr" => TreeChild::Expr(&stmt.expression)
        };
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "Expr",
                children,
            ),
        );
    }

    fn visit_let_stmt(&mut self, this: *const Stmt, stmt: &StmtLet) -> String {
        // 直接打印即可
        let ref_str = if stmt.is_ref {
            "Ref"
        } else {
            ""
        };
        let stmt_name = if let Some(ty) = &stmt.var_type {
            &format!("Let {} ({})", ref_str, ty)
        } else {
            &format!("Let {}", ref_str)
        };
        let children = if let Some(expr) = &stmt.init {
            indexmap! {
                "name" => TreeChild::Identifier(&stmt.name),
                "init" => TreeChild::Expr(expr),
            }
        } else {
            indexmap! {
                "name" => TreeChild::Identifier(&stmt.name),
            }
        };
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                stmt_name,
                children,
            ),
        );
    }

    fn visit_init_stmt(&mut self, this: *const Stmt, stmt: &StmtInit) -> String {
        let children = indexmap! {
            "name" => TreeChild::Identifier(&stmt.name),
            "init" => TreeChild::Expr(&stmt.init),
        };
        
        return format!(
            "STMT {:?} {}",
            this, 
            self.parenthesize(
                "Init",
                children,
            )
        );
    }

    fn visit_assign_stmt(&mut self, this: *const Stmt, stmt: &StmtAssign) -> String {
        let children = indexmap! {
            "vars" => TreeChild::ExprList(&stmt.assign_vars),
            "right" => TreeChild::Expr(&stmt.right_expr),
        };
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "Assign",
                children,
            )
        );
    }

    fn visit_block_stmt(&mut self, this: *const Stmt, stmt: &StmtBlock) -> String {
        let children = indexmap! {
            "statements" => TreeChild::StmtList(&stmt.statements),
        };
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "Block",
                children,
            )
        );
    }

    fn visit_if_stmt(&mut self, this: *const Stmt, stmt: &StmtIf) -> String {
        let mut children = indexmap! {
            "if_expr" => TreeChild::Expr(&stmt.if_branch.0),
            "if_chunk" => TreeChild::Stmt(&stmt.if_branch.1),
        };

        for (expr, chunk) in &stmt.else_if_branch {
            children.insert("else_if_expr", TreeChild::Expr(expr));
            children.insert("else_if_chunk", TreeChild::Stmt(chunk));
        }
        
        if let Some(chunk) = &stmt.else_branch {
            children.insert("else_chunk", TreeChild::Stmt(chunk));
        }
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "If",
                children,
            )
        );
    }

    fn visit_while_stmt(&mut self, this: *const Stmt, stmt: &StmtWhile) -> String {
        let mut children = indexmap! {
            "condition" => TreeChild::Expr(&stmt.condition),
            "chunk" => TreeChild::Stmt(&stmt.chunk),
        };
        
        if let Some(tag_name) = &stmt.tag {
            children.insert("tag", TreeChild::Tag(&tag_name));
        }
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "While",
                children,
            )
        );
    }

    fn visit_break_stmt(&mut self, this: *const Stmt, stmt: &StmtBreak) -> String {
        let children = if let Some(tag_name) = &stmt.tag {
            indexmap! {
                "tag" => TreeChild::Tag(&tag_name),
            }
        } else {
            indexmap!()
        };
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "Break",
                children,
            )
        );
    }

    fn visit_print_stmt(&mut self, this: *const Stmt, stmt: &StmtPrint) -> String {
        let children = if let Some(expr) = &stmt.expr {
            indexmap! {
                "vars" => TreeChild::Expr(expr),
            }
        } else {
            indexmap!()
        };
        
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "Print",
                children,
            )
        );
    }
}
