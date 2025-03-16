//! 语法树打印——语句打印模块

use indexmap::indexmap;

use crate::ast_printer::{AstPrinter, TreeChildren};
use crate::stmt::{Stmt, StmtAssign, StmtExpr, StmtInit, StmtLet, StmtPrint, StmtVisitor};

#[cfg(debug_assertions)]
impl StmtVisitor<String> for AstPrinter {
    fn visit_expr_stmt(&mut self, this: *const Stmt, stmt: &StmtExpr) -> String {
        let children = indexmap! {
            "expr" => TreeChildren::Expr(stmt.expression.as_ref())
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
                "name" => TreeChildren::Identifier(&stmt.name),
                "init" => TreeChildren::Expr(expr.as_ref()),
            }
        } else {
            indexmap! {
                "name" => TreeChildren::Identifier(&stmt.name),
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
            "name" => TreeChildren::Identifier(&stmt.name),
            "init" => TreeChildren::Expr(stmt.init.as_ref()),
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
            "vars" => TreeChildren::ExprList(&stmt.assign_vars),
            "right" => TreeChildren::Expr(stmt.right_expr.as_ref()),
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

    fn visit_print_stmt(&mut self, this: *const Stmt, stmt: &StmtPrint) -> String {
        let children = if let Some(expr) = &stmt.expr {
            indexmap! {
                "vars" => TreeChildren::Expr(expr.as_ref()),
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
