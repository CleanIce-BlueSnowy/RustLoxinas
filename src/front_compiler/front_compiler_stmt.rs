use crate::errors::error_types::CompileError;
use crate::expr_get_pos;
use crate::front_compiler::{BreakPatch, FrontCompiler};
use crate::instr::Instruction::*;
use crate::stmt::{Stmt, StmtAssign, StmtBlock, StmtBreak, StmtExpr, StmtIf, StmtInit, StmtLet, StmtPrint, StmtVisitor, StmtWhile};

impl<'a> StmtVisitor<Result<(), Vec<CompileError>>> for FrontCompiler<'a> {
    fn visit_expr_stmt(&mut self, _this: *const Stmt, stmt: &StmtExpr) -> Result<(), Vec<CompileError>> {
        let (expr_res, mut expr_code) = stmt.expression.accept(self)?;
        Self::pack_error(self.resolver.resolve_expr_stmt())?;
        let mut final_code = Self::pack_error(self.compiler.compile_expr_stmt(&mut expr_code, &expr_res))?;
        self.codes.append(&mut final_code);
        return Ok(());
    }

    fn visit_let_stmt(&mut self, _this: *const Stmt, stmt: &StmtLet) -> Result<(), Vec<CompileError>> {
        self.context.in_ref_let = stmt.is_ref;
        let (init_res, mut init_code) = if let Some(init) = &stmt.init {
            let (a, b) = init.accept(self)?;
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let (var_type, slot) = Self::pack_error(self.resolver.resolve_let_stmt(stmt, init_res.as_ref()))?;
        let mut final_code = Self::pack_error(self.compiler.compile_let_stmt(init_code.as_mut(), init_res.as_ref(), var_type, slot, self.context.in_loop))?;
        self.context.in_ref_let = false;
        self.codes.append(&mut final_code);
        return Ok(());
    }

    fn visit_init_stmt(&mut self, _this: *const Stmt, stmt: &StmtInit) -> Result<(), Vec<CompileError>> {
        let (init_res, mut init_code) = stmt.init.accept(self)?;
        let (var_type, slot, right_slot) = Self::pack_error(self.resolver.resolve_init_stmt(stmt, &init_res, self.context.in_loop))?;
        let mut final_code = Self::pack_error(self.compiler.compile_init_stmt(slot, right_slot, &mut init_code, &init_res, var_type))?;
        self.codes.append(&mut final_code);
        return Ok(());
    }

    fn visit_assign_stmt(&mut self, _this: *const Stmt, stmt: &StmtAssign) -> Result<(), Vec<CompileError>> {
        self.context.in_assign = true;
        let mut vars_res = Vec::with_capacity(stmt.assign_vars.len());
        let mut vars_code = Vec::with_capacity(stmt.assign_vars.len());
        for var in &stmt.assign_vars {
            let (var_res, var_code) = var.accept(self)?;
            vars_res.push(var_res);
            vars_code.push(var_code);
        }

        // 右侧表达式不要使用 in_assign 标志！
        self.context.in_assign = false;
        let (right_res, mut right_code) = stmt.right_expr.accept(self)?;
        self.context.in_assign = true;

        Self::pack_error(self.resolver.resolve_assign_stmt(stmt, &vars_res, &right_res))?;
        let mut final_code = Self::pack_error(self.compiler.compile_assign_stmt(&mut vars_code, &vars_res, &mut right_code, &right_res))?;
        self.context.in_assign = false;
        self.codes.append(&mut final_code);
        return Ok(());
    }

    fn visit_block_stmt(&mut self, _this: *const Stmt, stmt: &StmtBlock) -> Result<(), Vec<CompileError>> {
        let mut errors = vec![];
        
        self.resolver.enter_scope();
        
        self.compile_scope(&mut errors, &stmt.statements);
        
        let scope = self.resolver.leave_scope();
        
        // 单独的语句块不需要进行初始化一致性检查，所以直接初始化相关变量
        Self::scope_init_vars(&scope);
        
        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        };
    }

    fn visit_if_stmt(&mut self, _this: *const Stmt, stmt: &StmtIf) -> Result<(), Vec<CompileError>> {
        let (if_expr_res, mut if_expr_code) = stmt.if_branch.0.accept(self)?;
        let else_if_expr: Vec<_> = stmt.else_if_branch.iter().map(|(expr, _chunk)| expr.accept(self)).collect::<Result<_, _>>()?;
        let (else_if_expr_res, mut branch_expr_codes): (Vec<_>, Vec<_>) = else_if_expr.into_iter().unzip();

        self.resolver.resolve_if_stmt(stmt, &if_expr_res, &else_if_expr_res)?;

        let mut errors = vec![];
        let mut jump_end_locations = vec![];
        
        // if 分支
        self.codes.append(&mut if_expr_code);
        self.write_code(OpJumpFalsePop);
        let false_jump_location = self.codes.len();  // 待会回填地址
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);
        
        self.resolver.enter_scope();
        
        let if_chunk = if let Stmt::Block(temp) = stmt.if_branch.1.as_ref() { temp } else { unreachable!() };
        self.compile_scope(&mut errors, &if_chunk.statements);
        
        let compare_scope = self.resolver.leave_scope();
        
        self.write_code(OpJump);  // 跳转结尾
        jump_end_locations.push(self.codes.len());
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);
        
        let false_jump_dis = self.codes.len() - false_jump_location - 4;  // 算上地址
        self.codes[false_jump_location..(false_jump_location + 4)].copy_from_slice(&(false_jump_dis as u32).to_le_bytes());
        
        // else if 分支
        for (condition_code, (_chunk_expr, chunk_block)) in branch_expr_codes.iter_mut().zip(stmt.else_if_branch.iter()) {
            self.codes.append(condition_code);
            self.write_code(OpJumpFalsePop);
            let false_jump_location = self.codes.len();
            self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);
            
            self.resolver.enter_scope();
            
            let chunk = if let Stmt::Block(temp) = chunk_block { temp } else { unreachable!() };
            self.compile_scope(&mut errors, &chunk.statements);
            
            let this_scope = self.resolver.leave_scope();
            
            if !Self::scopes_same_inits(&compare_scope, &this_scope) {
                errors.push(CompileError::new(&chunk.pos, "All code branches must initialize variables in the same way.".to_string()));
            }
            
            // 小优化：若没有 else 分支，直接跳过这一步
            if let Some(_) = stmt.else_branch {
                self.write_code(OpJump);
                jump_end_locations.push(self.codes.len());
                self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);
            }
            
            let false_jump_dis = self.codes.len() - false_jump_location - 4;
            self.codes[false_jump_location..(false_jump_location + 4)].copy_from_slice(&(false_jump_dis as u32).to_le_bytes());
        }
        
        // else 分支
        if let Some(chunk_block) = &stmt.else_branch {
            self.resolver.enter_scope();
            
            let chunk = if let Stmt::Block(temp) = chunk_block.as_ref() { temp } else { unreachable!() };
            self.compile_scope(&mut errors, &chunk.statements);
            
            let this_scope = self.resolver.leave_scope();

            if !Self::scopes_same_inits(&compare_scope, &this_scope) {
                errors.push(CompileError::new(&chunk.pos, "All code branches must initialize variables in the same way.".to_string()));
            }
        }
        
        // 填充结尾跳转
        for jump_end_location in jump_end_locations {
            let jump_end_dis = self.codes.len() - jump_end_location - 4;
            self.codes[jump_end_location..(jump_end_location + 4)].copy_from_slice(&(jump_end_dis as u32).to_le_bytes());
        }
        
        Self::scope_init_vars(&compare_scope);

        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        };
    }

    fn visit_while_stmt(&mut self, _this: *const Stmt, stmt: &StmtWhile) -> Result<(), Vec<CompileError>> {
        let (condition_res, mut condition_code) = stmt.condition.accept(self)?;

        Self::pack_error(self.resolver.resolve_while_stmt(stmt, &condition_res))?;

        // 提前分配内存
        self.write_code(OpStackExtend);
        let alloc_location = self.codes.len();
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        let start_location = self.codes.len();

        // 写入条件
        self.codes.append(&mut condition_code);

        // 写入跳转
        self.write_code(OpJumpFalsePop);
        // 偏移占位符，并保存源地址
        let jump_location = self.codes.len();
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        let condition_end = self.codes.len();  // 一会用来计算循环体大小

        // 编译主体
        let before_slot = self.resolver.now_slot;  // 用于计算循环占用的空间
        self.resolver.enter_scope();
        self.context.in_loop = true;
        self.context.loop_tags.push(stmt.tag.clone());

        let mut errors = vec![];

        let chunk = if let Stmt::Block(temp) = stmt.chunk.as_ref() { temp } else { unreachable!() };
        self.compile_scope(&mut errors, &chunk.statements);

        let after_slot = self.resolver.now_slot;
        self.resolver.leave_scope();
        let this_tag = self.context.loop_tags.pop().unwrap();
        self.context.in_loop = !self.context.loop_tags.is_empty();  // 可能有嵌套的循环

        let jump_back = (self.codes.len() - start_location) as i32;
        self.write_code(OpJump);
        self.write_arg_dword((-jump_back - 5).to_le_bytes());  // 自己还有 5 字节！

        // 回填条件偏移地址
        let jump_end = self.codes.len() - condition_end;
        self.codes[jump_location..(jump_location + 4)].copy_from_slice(&(jump_end as u32).to_le_bytes());

        // 补充跳出循环偏移地址
        for patch in &self.break_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_end = self.codes.len() as u32 - patch.patch_pos - 4;  // 注意参数的 4 个字节
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)].copy_from_slice(&jump_end.to_le_bytes());
            }
        }
        self.break_patches.retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 分配与释放循环空间
        let memory_used = after_slot - before_slot;

        self.write_code(OpStackShrink);
        self.write_arg_dword((memory_used as u32).to_le_bytes());
        self.codes[alloc_location..(alloc_location + 4)].copy_from_slice(&(memory_used as u32).to_le_bytes());

        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        };
    }

    fn visit_break_stmt(&mut self, _this: *const Stmt, stmt: &StmtBreak) -> Result<(), Vec<CompileError>> {
        // 直接在这里进行分析并编译
        if !self.context.in_loop {
            return Err(vec![CompileError::new(&stmt.pos, "Cannot use 'break' outside a loop.".to_string())]);
        }
        if let Some(tag_name) = &stmt.tag {
            let mut found_tag = false;
            for tag in &self.context.loop_tags {
                if let Some(loop_tag) = tag {
                    if loop_tag == tag_name {
                        found_tag = true;
                        break;
                    }
                }
            }
            if !found_tag {
                return Err(vec![CompileError::new(&stmt.pos, format!("Undefined tag: @{}", tag_name))]);
            }
        }

        self.write_code(OpJump);
        let break_location = self.codes.len() as u32;
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);
        
        self.break_patches.push(BreakPatch::new(stmt.tag.clone(), break_location));
        
        return Ok(());
    }

    fn visit_print_stmt(&mut self, _this: *const Stmt, stmt: &StmtPrint) -> Result<(), Vec<CompileError>> {
        let (expr_res, expr_code, expr_pos) = if let Some(expr) = &stmt.expr {
            let (res, code) = expr.accept(self)?;
            (Some(res), Some(code), Some(expr_get_pos!(expr)))
        } else {
            (None, None, None)
        };
        Self::pack_error(self.resolver.resolve_print_stmt())?;
        let mut final_code = Self::pack_error(self.compiler.compile_print_stmt(expr_code, expr_res, expr_pos))?;
        self.codes.append(&mut final_code);
        return Ok(());
    }
}
