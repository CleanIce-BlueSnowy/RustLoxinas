use std::collections::LinkedList;

use crate::byte_handler::byte_writer::write_dword;
use crate::errors::error_types::CompileError;
use crate::expr_get_pos;
use crate::front_compiler::FrontCompiler;
use crate::instr::Instruction::*;
use crate::stmt::{Stmt, StmtAssign, StmtBlock, StmtExpr, StmtIf, StmtInit, StmtLet, StmtPrint, StmtVisitor, StmtWhile};

impl<'a> StmtVisitor<Result<LinkedList<u8>, Vec<CompileError>>> for FrontCompiler<'a> {
    fn visit_expr_stmt(&mut self, _this: *const Stmt, stmt: &StmtExpr) -> Result<LinkedList<u8>, Vec<CompileError>> {
        let (expr_res, mut expr_code) = stmt.expression.accept(self)?;
        Self::pack_error(self.resolver.resolve_expr_stmt())?;
        let final_code = Self::pack_error(self.compiler.compile_expr_stmt(&mut expr_code, &expr_res))?;
        return Ok(final_code);
    }

    fn visit_let_stmt(&mut self, _this: *const Stmt, stmt: &StmtLet) -> Result<LinkedList<u8>, Vec<CompileError>> {
        self.in_ref_let = stmt.is_ref;
        let (init_res, mut init_code) = if let Some(init) = &stmt.init {
            let (a, b) = init.accept(self)?;
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let var_type = Self::pack_error(self.resolver.resolve_let_stmt(stmt, init_res.as_ref()))?;
        let final_code = Self::pack_error(self.compiler.compile_let_stmt(init_code.as_mut(), init_res.as_ref(), var_type))?;
        self.in_ref_let = false;
        return Ok(final_code);
    }

    fn visit_init_stmt(&mut self, _this: *const Stmt, stmt: &StmtInit) -> Result<LinkedList<u8>, Vec<CompileError>> {
        let (init_res, mut init_code) = stmt.init.accept(self)?;
        let (var_type, slot) = Self::pack_error(self.resolver.resolve_init_stmt(stmt, &init_res, self.in_loop))?;
        let final_code = Self::pack_error(self.compiler.compile_init_stmt(slot, &mut init_code, &init_res, var_type))?;
        return Ok(final_code);
    }

    fn visit_assign_stmt(&mut self, _this: *const Stmt, stmt: &StmtAssign) -> Result<LinkedList<u8>, Vec<CompileError>> {
        self.in_assign = true;
        let mut vars_res = Vec::with_capacity(stmt.assign_vars.len());
        let mut vars_code = Vec::with_capacity(stmt.assign_vars.len());
        for var in &stmt.assign_vars {
            let (var_res, var_code) = var.accept(self)?;
            vars_res.push(var_res);
            vars_code.push(var_code);
        }

        // 右侧表达式不要使用 in_assign 标志！
        self.in_assign = false;
        let (right_res, mut right_code) = stmt.right_expr.accept(self)?;
        self.in_assign = true;

        Self::pack_error(self.resolver.resolve_assign_stmt(stmt, &vars_res, &right_res))?;
        let final_code = Self::pack_error(self.compiler.compile_assign_stmt(&mut vars_code, &vars_res, &mut right_code, &right_res))?;
        self.in_assign = false;
        return Ok(final_code);
    }

    fn visit_block_stmt(&mut self, _this: *const Stmt, stmt: &StmtBlock) -> Result<LinkedList<u8>, Vec<CompileError>> {
        let mut errors = vec![];
        let mut codes = LinkedList::new();
        
        self.resolver.enter_scope();
        
        self.compile_scope(&mut errors, &mut codes, &stmt.statements);
        
        let scope = self.resolver.leave_scope();
        
        // 单独的语句块不需要进行初始化一致性检查，所以直接初始化相关变量
        for &variable in &scope.init_vars {
            // SAFETY: Scope 的 init_vars 一定引用的是上一层作用域的变量，所以安全
            unsafe {
                (*variable).initialized = true;
            }
        }
        
        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(codes)
        };
    }

    fn visit_if_stmt(&mut self, _this: *const Stmt, stmt: &StmtIf) -> Result<LinkedList<u8>, Vec<CompileError>> {
        let (if_expr_res, if_expr_code) = stmt.if_case.0.accept(self)?;
        let else_if_expr: Vec<_> = stmt.else_if_cases.iter().map(|(expr, _chunk)| expr.accept(self)).collect::<Result<_, _>>()?;
        let (else_if_expr_res, mut branch_expr_codes): (Vec<_>, Vec<_>) = else_if_expr.into_iter().unzip();

        self.resolver.resolve_if_stmt(stmt, &if_expr_res, &else_if_expr_res)?;

        // if 语句的编译和分析交替进行（需要调用 self.compile_scope 和 self.resolver 的 enter_scope 与 leave_scope），因此直接在此处编译
        // 倒序编译技术，方便填写跳转指令的相对跳转参数

        let mut target = LinkedList::new();
        let mut errors = vec![];
        let mut end_cnt: u32 = 0;
        let mut compare_target = None;

        if let Some(else_chunk) = &stmt.else_case {
            let block_chunk = if let Stmt::Block(temp) = else_chunk.as_ref() { temp } else { panic!("Invalid.") };
            let mut codes = LinkedList::new();

            self.resolver.enter_scope();

            self.compile_scope(&mut errors, &mut codes, &block_chunk.statements);

            let scope = self.resolver.leave_scope();
            compare_target = Some(scope.init_vars);

            end_cnt += codes.len() as u32;

            // 在前端插入，并避免复制
            codes.append(&mut target);
            target = codes;
        }

        // 倒序生成，将 if 分支也加入进去，避免重复代码
        let mut branch_chunks: Vec<_> = stmt.else_if_cases.iter().map(|(_expr, chunk)| chunk).collect();
        branch_chunks.reverse();
        branch_chunks.push(stmt.if_case.1.as_ref());
        branch_expr_codes.reverse();
        branch_expr_codes.push(if_expr_code);
        for (expr, chunk) in branch_expr_codes.into_iter().zip(branch_chunks.into_iter()) {
            let mut expr_code: LinkedList<u8> = expr;  // 避免 building 导致的错误
            let block_chunk = if let Stmt::Block(temp) = chunk { temp } else { panic!("Invalid.") };
            let mut codes = LinkedList::new();

            self.resolver.enter_scope();

            self.compile_scope(&mut errors, &mut codes, &block_chunk.statements);

            let scope = self.resolver.leave_scope();
            if let Some(compare) = &compare_target {  // 判断子代码块一致性
                if compare != &scope.init_vars {
                    errors.push(CompileError::new(&block_chunk.pos, "Mismatched initialization across branches.".to_string()));
                }
            } else {
                compare_target = Some(scope.init_vars);
            }

            // 分支结尾跳转
            if end_cnt != 0 {
                codes.push_back(OpJump.into());
                write_dword(&mut codes, end_cnt.to_le_bytes());
            }

            // 分支不符合时跳转
            let else_cnt = codes.len() as u32;
            let mut temp: LinkedList<_> = [OpJumpFalsePop.into()].into_iter().collect();
            write_dword(&mut temp, else_cnt.to_le_bytes());
            temp.append(&mut codes);
            codes = temp;

            // 写入判断条件
            expr_code.append(&mut codes);
            codes = expr_code;

            // 更新结尾跳转
            end_cnt += codes.len() as u32;

            // 写入代码
            codes.append(&mut target);
            target = codes;
        }

        // 初始化变量
        for variables in compare_target.unwrap() {
            // SAFETY: 不必多言
            unsafe {
                (*variables).initialized = true;
            }
        }

        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(target)
        };
    }

    fn visit_while_stmt(&mut self, _this: *const Stmt, stmt: &StmtWhile) -> Result<LinkedList<u8>, Vec<CompileError>> {
        let (condition_res, mut condition_code) = stmt.condition.accept(self)?;

        Self::pack_error(self.resolver.resolve_while_stmt(stmt, &condition_res))?;

        let mut codes = LinkedList::new();

        // 写入条件
        codes.append(&mut condition_code);

        // 写入跳转
        codes.push_back(OpJumpFalsePop.into());
        // 偏移占位符，并保存源地址
        let jump_access: Vec<_> = [0x00, 0x00, 0x00, 0x00].into_iter().map(|value| {
            codes.push_back(value);
            return codes.back_mut().unwrap() as *mut u8
        }).collect();

        let condition_length = codes.len() as u32;  // 一会用来计算循环体大小

        // 编译主体
        let mut errors = vec![];

        let before_slot = self.resolver.now_slot;  // 用于计算循环占用的空间
        self.resolver.enter_scope();
        self.in_loop = true;

        let block = if let Stmt::Block(temp) = stmt.chunk.as_ref() { temp } else { panic!("Invalid.") };
        self.compile_scope(&mut errors, &mut codes, &block.statements);

        self.in_loop = false;
        let after_slot = self.resolver.now_slot;
        self.resolver.leave_scope();
        
        // 释放循环空间（小优化：若无占用空间则不用释放）
        let memory_used = after_slot - before_slot;
        if memory_used != 0 {
            codes.push_back(OpStackShrink.into());
            write_dword(&mut codes, (memory_used as u32).to_le_bytes());
        }

        let jump_back = codes.len() as u32;
        codes.push_back(OpJump.into());
        write_dword(&mut codes, (-(jump_back as i32 + 5)).to_le_bytes());  // 别忘了自己还有 5 个字节！

        // 回填条件偏移地址
        let jump_end = codes.len() as u32 - condition_length;
        for (access, byte) in jump_access.into_iter().zip(jump_end.to_le_bytes().into_iter()) {
            // SAFETY: 一定指向链表节点
            unsafe {
                *access = byte;
            }
        }

        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(codes)
        };
    }

    fn visit_print_stmt(&mut self, _this: *const Stmt, stmt: &StmtPrint) -> Result<LinkedList<u8>, Vec<CompileError>> {
        let (expr_res, expr_code, expr_pos) = if let Some(expr) = &stmt.expr {
            let (res, code) = expr.accept(self)?;
            (Some(res), Some(code), Some(expr_get_pos!(expr)))
        } else {
            (None, None, None)
        };
        Self::pack_error(self.resolver.resolve_print_stmt())?;
        let final_code = Self::pack_error(self.compiler.compile_print_stmt(expr_code, expr_res, expr_pos))?;
        return Ok(final_code);
    }
}
