//! 语法树打印——语句打印模块

use indexmap::indexmap;

use crate::ast_printer::{AstPrinter, TreeChild};
use crate::stmt::{Stmt, StmtExpr, StmtLet, StmtVisitor};

#[cfg(debug_assertions)]
impl StmtVisitor<String> for AstPrinter {
    fn visit_expr_stmt(&mut self, this: *const Stmt, stmt: &StmtExpr) -> String {
        // 直接打印即可
        return format!(
            "STMT {:?} {}",
            this,
            self.parenthesize(
                "Expr",
                indexmap! {
                    "expr" => TreeChild::Expr(stmt.expression.as_ref())
                },
            ),
        );
    }

    fn visit_let_stmt(&mut self, this: *const Stmt, stmt: &StmtLet) -> String {
        // 直接打印即可
        let stmt_name = if let Some(ty) = &stmt.var_type {
            &format!("Let ({})", ty)
        } else {
            "Let"
        };
        let children = if let Some(expr) = &stmt.init {
            indexmap! {
                "name" => TreeChild::Identifier(&stmt.name),
                "init" => TreeChild::Expr(expr.as_ref()),
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
}
