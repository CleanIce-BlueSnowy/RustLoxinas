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
    /// 块语句
    Block(StmtBlock),
    /// 条件判断语句
    If(StmtIf),
    /// 条件循环语句
    While(StmtWhile),
    /// 临时辅助功能：打印语句
    Print(StmtPrint),
}

/// 表达式语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtExpr {
    /// 位置信息
    pub pos: Position,
    /// 表达式
    pub expression: Expr,
}

/// 变量定义语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtLet {
    /// 位置信息
    pub pos: Position,
    /// `let` 关键字所在位置
    pub let_pos: Position,
    /// 名称位置信息
    pub name_pos: Position,
    /// 变量名称
    pub name: String,
    /// 变量类型
    pub var_type: Option<TypeTag>,
    /// 初始化表达式
    pub init: Option<Expr>,
    /// 是否为引用
    pub is_ref: bool,
}

/// 变量延迟初始化语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtInit {
    /// 位置信息
    pub pos: Position,
    /// 名称位置信息
    pub name_pos: Position,
    /// 变量名称
    pub name: String,
    /// 初始化表达式
    pub init: Expr,
}

/// 变量赋值语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtAssign {
    /// 位置信息
    pub pos: Position,
    /// 赋值变量
    pub assign_vars: Vec<Expr>,
    /// 赋值源表达式
    pub right_expr: Expr,
}

/// 块语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtBlock {
    /// 位置信息
    pub pos: Position,
    /// 子句
    pub statements: Vec<Stmt>,
}

/// 条件判断语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtIf {
    pub pos: Position,
    pub if_case: (Expr, Box<Stmt>),
    pub else_if_cases: Vec<(Expr, Stmt)>,
    pub else_case: Option<Box<Stmt>>,
}

/// 条件循环语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtWhile {
    pub pos: Position,
    pub condition: Expr,
    pub chunk: Box<Stmt>,
}

/// 临时辅助功能：打印语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtPrint {
    /// 位置信息
    pub pos: Position,
    /// 源表达式
    pub expr: Option<Expr>,
}

/** 使用访问者模式的访问器，用于访问各种语句，从而访问抽象语法树

`RetType` 是返回类型
 */
pub trait StmtVisitor<RetType> {
    #[must_use]
    fn visit_expr_stmt(&mut self, this: *const Stmt, stmt: &StmtExpr) -> RetType;
    #[must_use]
    fn visit_let_stmt(&mut self, this: *const Stmt, stmt: &StmtLet) -> RetType;
    #[must_use]
    fn visit_init_stmt(&mut self, this: *const Stmt, stmt: &StmtInit) -> RetType;
    #[must_use]
    fn visit_assign_stmt(&mut self, this: *const Stmt, stmt: &StmtAssign) -> RetType;
    #[must_use]
    fn visit_block_stmt(&mut self, this: *const Stmt, stmt: &StmtBlock) -> RetType;
    #[must_use]
    fn visit_if_stmt(&mut self, this: *const Stmt, stmt: &StmtIf) -> RetType;
    #[must_use]
    fn visit_while_stmt(&mut self, this: *const Stmt, stmt: &StmtWhile) -> RetType;
    #[must_use]
    fn visit_print_stmt(&mut self, this: *const Stmt, stmt: &StmtPrint) -> RetType;
}

impl Stmt {
    /// 访问自己，通过模式匹配具体的枚举值
    #[must_use]
    pub fn accept<RetType>(&self, visitor: &mut dyn StmtVisitor<RetType>) -> RetType {
        let ptr = self as *const Stmt;
        return match self {
            Stmt::Expr(stmt) => visitor.visit_expr_stmt(ptr, stmt),
            Stmt::Let(stmt) => visitor.visit_let_stmt(ptr, stmt),
            Stmt::Init(stmt) => visitor.visit_init_stmt(ptr, stmt),
            Stmt::Assign(stmt) => visitor.visit_assign_stmt(ptr, stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(ptr, stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(ptr, stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(ptr, stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(ptr, stmt),
        };
    }
}

/// 获取语句的位置信息
#[macro_export]
macro_rules! stmt_get_pos {
    ( $expression:expr ) => {
        {
            use crate::stmt::Stmt;
            match $expression {
                Stmt::Expr(stmt) => stmt.pos.clone(),
                Stmt::Let(stmt) => stmt.pos.clone(),
                Stmt::Init(stmt) => stmt.pos.clone(),
                Stmt::Assign(stmt) => stmt.pos.clone(),
                Stmt::Block(stmt) => stmt.pos.clone(),
                Stmt::If(stmt) => stmt.pos.clone(),
                Stmt::While(stmt) => stmt.pos.clone(),
                Stmt::Print(stmt) => stmt.pos.clone(),
            }
        }
    }
}
