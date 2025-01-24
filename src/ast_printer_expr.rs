use std::rc::Rc;
use crate::ast_printer::AstPrinter;
use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::{Expr, ExprVisitor};
use crate::tokens::Token;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) -> String {
        let name = self.operator_to_string(&operator.token_type);
        return self.parenthesize(&name, &[left, right]);
    }

    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> String {
        return self.parenthesize("group", &[expr]);
    }

    fn visit_literal_expr(&mut self, value: &Data) -> String {
        return match value {  // 将数据转换为对应的字符串，并带上 Loxinas 代码对应的数据后缀，字符串需要处理
            Data::Bool(res) => res.to_string(),
            Data::String(res) => {  // 处理字符串
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
                ret
            }
            Data::Float(float) => {
                match float {
                    DataFloat::Float(res) => format!("{}f", res.to_string()),
                    DataFloat::Double(res) => res.to_string(),
                }
            }
            Data::Integer(integer) => {
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
            }
            Data::Char(ch) => {
                format!("'{}'", match ch {
                    '"' => r#"""#.to_string(),
                    '\n' => r"\n".to_string(),
                    '\0' => r"\0".to_string(),
                    '\t' => r"\t".to_string(),
                    '\r' => r"\r".to_string(),
                    '\\' => r"\\".to_string(),
                    '\'' => r"\'".to_string(),
                    _ => ch.to_string(),
                })
            }
        }
    }

    fn visit_unary_expr(&mut self, operator: &Rc<Token>, right: &Box<Expr>) -> String {
        let name = self.operator_to_string(&operator.token_type);
        return self.parenthesize(&name, &[right]);
    }
}