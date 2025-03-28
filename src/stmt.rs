//! 语句模块

use crate::expr::Expr;
use crate::position::Position;
use crate::types::TypeTag;

/** 语句

使用访问者模式
 */
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Stmt {
    /// 表达式
    Expr(StmtExpr),
    /// 变量定义
    Let(StmtLet),
    /// 变量延迟初始化
    Init(StmtInit),
    /// 变量赋值
    Assign(StmtAssign),
    /// 临时辅助功能：打印语句
    Print(StmtPrint),
}

/// 表达式语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtExpr {
    /// 表达式
    pub expression: Box<Expr>,
}

/// 变量定义语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtLet {
    /// `let` 关键字所在位置
    pub let_pos: Position,
    /// 名称位置信息
    pub name_pos: Position,
    /// 变量名称
    pub name: String,
    /// 变量类型
    pub var_type: Option<TypeTag>,
    /// 初始化表达式
    pub init: Option<Box<Expr>>,
    /// 是否为引用
    pub is_ref: bool,
}

/// 变量延迟初始化语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtInit {
    /// 名称位置信息
    pub name_pos: Position,
    /// 变量名称
    pub name: String,
    /// 初始化表达式
    pub init: Box<Expr>,
}

/// 变量赋值语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtAssign {
    /// 赋值变量
    pub assign_vars: Vec<Box<Expr>>,
    /// 赋值源表达式
    pub right_expr: Box<Expr>,
}

/// 临时辅助功能：打印语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtPrint {
    /// 源表达式
    pub expr: Option<Box<Expr>>,
}

/** 使用访问者模式的访问器，用于访问各种语句，从而访问抽象语法树

`RetType` 是返回类型
 */
pub trait StmtVisitor<RetType> {
    /// 访问表达式语句
    fn visit_expr_stmt(&mut self, this: *const Stmt, stmt: &StmtExpr) -> RetType;
    fn visit_let_stmt(&mut self, this: *const Stmt, stmt: &StmtLet) -> RetType;
    fn visit_init_stmt(&mut self, this: *const Stmt, stmt: &StmtInit) -> RetType;
    fn visit_assign_stmt(&mut self, this: *const Stmt, stmt: &StmtAssign) -> RetType;
    fn visit_print_stmt(&mut self, this: *const Stmt, stmt: &StmtPrint) -> RetType;
}

impl Stmt {
    /// 访问自己，通过模式匹配具体的枚举值
    pub fn accept<RetType>(&self, visitor: &mut dyn StmtVisitor<RetType>) -> RetType {
        let ptr = self as *const Stmt;
        return match self {
            Stmt::Expr(stmt) => visitor.visit_expr_stmt(ptr, stmt),
            Stmt::Let(stmt) => visitor.visit_let_stmt(ptr, stmt),
            Stmt::Init(stmt) => visitor.visit_init_stmt(ptr, stmt),
            Stmt::Assign(stmt) => visitor.visit_assign_stmt(ptr, stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(ptr, stmt),
        };
    }
}
