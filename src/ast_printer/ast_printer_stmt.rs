//! 语法树打印——语句打印模块

use indexmap::indexmap;

use crate::ast_printer::{AstPrinter, TreeChild};
use crate::stmt::{StmtAssign, StmtBlock, StmtBreak, StmtContinue, StmtEmpty, StmtExpr, StmtFor, StmtFunc, StmtIf, StmtInit, StmtLet, StmtLoop, StmtReturn, StmtVisitor, StmtWhile};

#[cfg(debug_assertions)]
impl StmtVisitor<String> for AstPrinter {
    fn visit_empty_stmt(&mut self, _stmt: &StmtEmpty) -> String {
        let children = indexmap!();

        format!("STMT {}", self.parenthesize("Empty", children,),)
    }

    fn visit_expr_stmt(&mut self, stmt: &StmtExpr) -> String {
        let children = indexmap! {
            "expr" => TreeChild::Expr(&stmt.expression)
        };

        format!("STMT {}", self.parenthesize("Expr", children,),)
    }

    fn visit_let_stmt(&mut self, stmt: &StmtLet) -> String {
        // 直接打印即可
        let ref_str = if stmt.is_ref { "Ref" } else { "" };
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
        format!(
            "STMT {}",
            self.parenthesize(stmt_name, children,),
        )
    }

    fn visit_init_stmt(&mut self, stmt: &StmtInit) -> String {
        let children = indexmap! {
            "name" => TreeChild::Identifier(&stmt.name),
            "init" => TreeChild::Expr(&stmt.init),
        };

        format!("STMT {}", self.parenthesize("Init", children,),)
    }

    fn visit_assign_stmt(&mut self, stmt: &StmtAssign) -> String {
        let children = indexmap! {
            "vars"  => TreeChild::ExprList(&stmt.assign_vars),
            "right" => TreeChild::Expr(&stmt.right_expr),
        };

        format!("STMT {}", self.parenthesize("Assign", children,),)
    }

    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> String {
        let children = indexmap! {
            "statements" => TreeChild::StmtList(&stmt.statements),
        };

        format!("STMT {}", self.parenthesize("Block", children,),)
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> String {
        let mut children = indexmap! {
            "if_expr"   => TreeChild::Expr(&stmt.if_branch.0),
            "if_chunk"  => TreeChild::Stmt(&stmt.if_branch.1),
        };

        for (expr, chunk) in &stmt.else_if_branch {
            children.insert("else_if_expr", TreeChild::Expr(expr));
            children.insert("else_if_chunk", TreeChild::Stmt(chunk));
        }

        if let Some(chunk) = &stmt.else_branch {
            children.insert("else_chunk", TreeChild::Stmt(chunk));
        }

        format!("STMT {}", self.parenthesize("If", children,),)
    }

    fn visit_loop_stmt(&mut self, stmt: &StmtLoop) -> String {
        let mut children = if let Some(tag_name) = &stmt.tag {
            indexmap! {
                "tag" => TreeChild::Tag(&tag_name),
            }
        } else {
            indexmap!()
        };

        children.insert("chunk", TreeChild::Stmt(&stmt.chunk));

        format!("STMT {}", self.parenthesize("Loop", children,),)
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> String {
        let mut children = indexmap! {
            "condition" => TreeChild::Expr(&stmt.condition),
        };

        if let Some(tag_name) = &stmt.tag {
            children.insert("tag", TreeChild::Tag(&tag_name));
        }

        children.insert("chunk", TreeChild::Stmt(&stmt.chunk));

        format!("STMT {}", self.parenthesize("While", children,),)
    }

    fn visit_for_stmt(&mut self, stmt: &StmtFor) -> String {
        let mut children = indexmap! {
            "init"      => TreeChild::Stmt(&stmt.init),
            "condition" => TreeChild::Expr(&stmt.condition),
            "update"    => TreeChild::Stmt(&stmt.update),
        };

        if let Some(tag_name) = &stmt.tag {
            children.insert("tag", TreeChild::Tag(&tag_name));
        }

        children.insert("chunk", TreeChild::Stmt(&stmt.chunk));

        format!("STMT {}", self.parenthesize("For", children,),)
    }

    fn visit_break_stmt(&mut self, stmt: &StmtBreak) -> String {
        let children = if let Some(tag_name) = &stmt.tag {
            indexmap! {
                "tag" => TreeChild::Tag(&tag_name),
            }
        } else {
            indexmap!()
        };

        format!("STMT {}", self.parenthesize("Break", children,),)
    }

    fn visit_continue_stmt(&mut self, stmt: &StmtContinue) -> String {
        let children = if let Some(tag_name) = &stmt.tag {
            indexmap! {
                "tag" => TreeChild::Tag(&tag_name),
            }
        } else {
            indexmap!()
        };

        format!(
            "STMT {}",
            self.parenthesize("Continue", children,),
        )
    }

    fn visit_func_stmt(&mut self, stmt: &StmtFunc) -> String {
        let mut children = indexmap! {
            "name" => TreeChild::Identifier(&stmt.name),
            "params" => TreeChild::ParamList(&stmt.params),
        };
        
        let string;  // 防止引用目标生命周期不够，移到 if 之外
        if let Some(return_type) = &stmt.return_type {
            string = return_type.to_string();
            children.insert("return_type", TreeChild::Identifier(&string));
        }
        
        children.insert("body", TreeChild::Stmt(&stmt.body));
        
        format!("STMT {}", self.parenthesize("Func", children))
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> String {
        let children = if let Some(expr) = &stmt.expr {
            indexmap! {
                "expr" => TreeChild::Expr(expr),
            }
        } else {
            indexmap!()
        };
        
        format!("STMT {}", self.parenthesize("Return", children))
    }
}
