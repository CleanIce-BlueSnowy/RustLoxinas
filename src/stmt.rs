//! 语句模块

use crate::expr::Expr;
use crate::position::Position;
use crate::types::TypeTag;

/** 语句

使用访问者模式
 */
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Stmt {
    /// 空语句
    Empty(Box<StmtEmpty>),
    /// 表达式
    Expr(Box<StmtExpr>),
    /// 变量定义
    Let(Box<StmtLet>),
    /// 变量延迟初始化
    Init(Box<StmtInit>),
    /// 变量赋值
    Assign(Box<StmtAssign>),
    /// 块语句
    Block(Box<StmtBlock>),
    /// 条件判断语句
    If(Box<StmtIf>),
    /// 无限循环语句
    Loop(Box<StmtLoop>),
    /// 条件循环语句
    While(Box<StmtWhile>),
    /// 迭代循环语句
    For(Box<StmtFor>),
    /// 退出循环语句
    Break(Box<StmtBreak>),
    /// 继续循环语句
    Continue(Box<StmtContinue>),
    /// 临时辅助功能：打印语句
    Print(Box<StmtPrint>),
}

/// 空语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtEmpty {
    pub pos: Position,
}

/// 表达式语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtExpr {
    pub pos: Position,
    pub expression: Expr,
}

/// 变量定义语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtLet {
    pub pos: Position,
    pub let_pos: Position,
    pub name_pos: Position,
    pub name: String,
    pub var_type: Option<TypeTag>,
    pub init: Option<Expr>,
    pub is_ref: bool,
}

/// 变量延迟初始化语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtInit {
    pub pos: Position,
    pub name_pos: Position,
    pub name: String,
    pub init: Expr,
}

/// 变量赋值语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtAssign {
    pub pos: Position,
    pub assign_vars: Vec<Expr>,
    pub right_expr: Expr,
}

/// 块语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtBlock {
    pub pos: Position,
    pub statements: Vec<Stmt>,
}

/// 条件判断语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtIf {
    pub pos: Position,
    pub if_branch: (Expr, Stmt),
    pub else_if_branch: Vec<(Expr, Stmt)>,
    pub else_branch: Option<Stmt>,
}

/// 无限循环语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtLoop {
    pub pos: Position,
    pub chunk: Stmt,
    pub tag: Option<String>,
}

/// 条件循环语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtWhile {
    pub pos: Position,
    pub condition: Expr,
    pub chunk: Stmt,
    pub tag: Option<String>,
}

/// 迭代循环语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtFor {
    pub pos: Position,
    pub init: Stmt,
    pub condition: Expr,
    pub update: Stmt,
    pub chunk: Stmt,
    pub tag: Option<String>,
}

/// 退出循环语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtBreak {
    pub pos: Position,
    pub tag: Option<String>,
}

/// 继续循环语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtContinue {
    pub pos: Position,
    pub tag: Option<String>,
}

/// 临时辅助功能：打印语句
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StmtPrint {
    pub pos: Position,
    pub expr: Option<Expr>,
}

/** 使用访问者模式的访问器，用于访问各种语句，从而访问抽象语法树

`RetType` 是返回类型
 */
pub trait StmtVisitor<RetType> {
    #[must_use]
    fn visit_empty_stmt(&mut self, this: *const Stmt, stmt: &StmtEmpty) -> RetType;
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
    fn visit_loop_stmt(&mut self, this: *const Stmt, stmt: &StmtLoop) -> RetType;
    #[must_use]
    fn visit_while_stmt(&mut self, this: *const Stmt, stmt: &StmtWhile) -> RetType;
    #[must_use]
    fn visit_for_stmt(&mut self, this: *const Stmt, stmt: &StmtFor) -> RetType;
    #[must_use]
    fn visit_break_stmt(&mut self, this: *const Stmt, stmt: &StmtBreak) -> RetType;
    #[must_use]
    fn visit_continue_stmt(&mut self, this: *const Stmt, stmt: &StmtContinue) -> RetType;
    #[must_use]
    fn visit_print_stmt(&mut self, this: *const Stmt, stmt: &StmtPrint) -> RetType;
}

impl Stmt {
    /// 访问自己，通过模式匹配具体的枚举值
    #[must_use]
    pub fn accept<RetType>(&self, visitor: &mut impl StmtVisitor<RetType>) -> RetType {
        let ptr = self as *const Stmt;
        return match self {
            Stmt::Empty(stmt) => visitor.visit_empty_stmt(ptr, stmt),
            Stmt::Expr(stmt) => visitor.visit_expr_stmt(ptr, stmt),
            Stmt::Let(stmt) => visitor.visit_let_stmt(ptr, stmt),
            Stmt::Init(stmt) => visitor.visit_init_stmt(ptr, stmt),
            Stmt::Assign(stmt) => visitor.visit_assign_stmt(ptr, stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(ptr, stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(ptr, stmt),
            Stmt::Loop(stmt) => visitor.visit_loop_stmt(ptr, stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(ptr, stmt),
            Stmt::For(stmt) => visitor.visit_for_stmt(ptr, stmt),
            Stmt::Break(stmt) => visitor.visit_break_stmt(ptr, stmt),
            Stmt::Continue(stmt) => visitor.visit_continue_stmt(ptr, stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(ptr, stmt),
        };
    }
}

/// 获取语句的位置信息
#[macro_export]
macro_rules! stmt_get_pos {
    ( $expression:expr ) => {{
        use crate::stmt::Stmt;
        match $expression {
            Stmt::Empty(stmt) => stmt.pos.clone(),
            Stmt::Expr(stmt) => stmt.pos.clone(),
            Stmt::Let(stmt) => stmt.pos.clone(),
            Stmt::Init(stmt) => stmt.pos.clone(),
            Stmt::Assign(stmt) => stmt.pos.clone(),
            Stmt::Block(stmt) => stmt.pos.clone(),
            Stmt::If(stmt) => stmt.pos.clone(),
            Stmt::Loop(stmt) => stmt.pos.clone(),
            Stmt::While(stmt) => stmt.pos.clone(),
            Stmt::For(stmt) => stmt.pos.clone(),
            Stmt::Break(stmt) => stmt.pos.clone(),
            Stmt::Continue(stmt) => stmt.pos.clone(),
            Stmt::Print(stmt) => stmt.pos.clone(),
        }
    }};
}
