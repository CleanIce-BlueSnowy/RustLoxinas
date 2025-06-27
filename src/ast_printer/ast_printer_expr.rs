//! 语法树打印——表达式打印模块

use indexmap::indexmap;

use crate::ast_printer::{AstPrinter, TreeChild};
use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::{ExprAs, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprUnary, ExprVariable, ExprVisitor};

#[cfg(debug_assertions)]
impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> String {
        let name = format!(
            "Binary {}",
            self.operator_to_string(&expr.operator.token_type)
        );
        let children = indexmap! {
            "left"  => TreeChild::Expr(&expr.left),
            "right" => TreeChild::Expr(&expr.right),
        };

        format!("EXPR {}", self.parenthesize(&name, children,),)
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> String {
        let children = indexmap! {
            "expr" => TreeChild::Expr(&expr.expression),
        };

        format!("EXPR {}", self.parenthesize("Group", children,),)
    }

    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> String {
        match &expr.value {
            // 将数据转换为对应的字符串，并带上 Loxinas 代码对应的数据后缀，字符串需要处理
            Data::Bool(res) => res.to_string(),
            Data::String(res) => {
                // 处理字符串
                let mut ret = String::new();
                ret.push('"');
                for ch in res.clone().chars() {
                    match ch {
                        '"' => ret.push_str(r#"\""#),
                        '\n' => ret.push_str(r"\n"),
                        '\0' => ret.push_str(r"\0"),
                        '\t' => ret.push_str(r"\t"),
                        '\r' => ret.push_str(r"\r"),
                        '\\' => ret.push_str(r"\"),
                        '\'' => ret.push_str(r"'"),
                        _ => ret.push(ch),
                    }
                }
                format!("EXPR Literal {}", ret)
            }
            Data::Float(float) => {
                format!(
                    "EXPR Literal {}",
                            match float {
                        DataFloat::Float(res) => format!("{}f", res.to_string()),
                        DataFloat::Double(res) => res.to_string(),
                    }
                )
            }
            Data::Integer(integer) => {
                format!(
                    "EXPR Literal {}",
                            match integer {
                        DataInteger::Byte(res) => format!("{}b", res.to_string()),
                        DataInteger::SByte(res) => format!("{}sb", res.to_string()),
                        DataInteger::Short(res) => format!("{}s", res.to_string()),
                        DataInteger::UShort(res) => format!("{}us", res.to_string()),
                        DataInteger::Int(res) => res.to_string(),
                        DataInteger::UInt(res) => format!("{}u", res.to_string()),
                        DataInteger::Long(res) => format!("{}l", res.to_string()),
                        DataInteger::ULong(res) => format!("{}ul", res.to_string()),
                        DataInteger::ExtInt(res) => format!("{}e", res.to_string()),
                        DataInteger::UExtInt(res) => format!("{}ue", res.to_string()),
                    }
                )
            }
            Data::Char(ch) => {
                format!(
                    "EXPR Literal '{}'",
                            match ch {
                        '"' => r#"""#.to_string(),
                        '\n' => r"\n".to_string(),
                        '\0' => r"\0".to_string(),
                        '\t' => r"\t".to_string(),
                        '\r' => r"\r".to_string(),
                        '\\' => r"\\".to_string(),
                        '\'' => r"\'".to_string(),
                        _ => ch.to_string(),
                    }
                )
            }
        }
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> String {
        let name = format!(
            "Unary {}",
            self.operator_to_string(&expr.operator.token_type)
        );
        let children = indexmap! {
            "right" => TreeChild::Expr(&expr.right),
        };

        format!("EXPR {}", self.parenthesize(&name, children,),)
    }

    fn visit_as_expr(&mut self, expr: &ExprAs) -> String {
        let name = format!("As => {}", expr.target);
        let children = indexmap! {
            "expr" => TreeChild::Expr(&expr.expression),
        };

        format!("EXPR {}", self.parenthesize(&name, children,),)
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> String {
        let children = indexmap! {
            "name" => TreeChild::Identifier(&expr.name),
        };

        format!(
            "EXPR {}",
            self.parenthesize("Variable", children,),
        )
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) -> String {
        let children = indexmap! {
            "call_target" => TreeChild::Identifier(&expr.func_name),
            "arguments" => TreeChild::ExprList(&expr.arguments),
        };
        
        format!(
            "EXPR {}",
            self.parenthesize("Call", children),
        )
    }
}
