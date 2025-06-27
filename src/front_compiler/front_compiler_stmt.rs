use crate::errors::error_types::{CompileError, CompileResultList};
use crate::front_compiler::{BreakPatch, ContinuePatch, FrontCompiler};
use crate::instr::Instruction::*;
use crate::stmt::{Stmt, StmtAssign, StmtBlock, StmtBreak, StmtContinue, StmtEmpty, StmtExpr, StmtFor, StmtFunc, StmtIf, StmtInit, StmtLet, StmtLoop, StmtReturn, StmtVisitor, StmtWhile};
use crate::types::ValueType;
use crate::stmt_get_pos;
use std::slice;
use crate::data::DataSize;

impl<'a> StmtVisitor<CompileResultList<()>> for FrontCompiler<'a> {
    fn visit_empty_stmt(&mut self, _stmt: &StmtEmpty) -> CompileResultList<()> {
        Ok(())
    }

    fn visit_expr_stmt(&mut self, stmt: &StmtExpr) -> CompileResultList<()> {
        let (expr_res, mut expr_code) = stmt.expression.accept(self)?;
        Self::pack_error(self.resolver.resolve_expr_stmt())?;
        let mut final_code =
            Self::pack_error(self.compiler.compile_expr_stmt(&mut expr_code, &expr_res))?;
        self.codes.append(&mut final_code);
        Ok(())
    }

    fn visit_let_stmt(&mut self, stmt: &StmtLet) -> CompileResultList<()> {
        self.context.in_ref_let = stmt.is_ref;
        let (init_res, mut init_code) = if let Some(init) = &stmt.init {
            let (a, b) = init.accept(self)?;
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let (var_type, slot) =
            Self::pack_error(self.resolver.resolve_let_stmt(stmt, init_res.as_ref()))?;
        let mut final_code = Self::pack_error(self.compiler.compile_let_stmt(
            init_code.as_mut(),
            init_res.as_ref(),
            var_type,
            slot,
            self.context.in_loop,
        ))?;
        self.context.in_ref_let = false;
        self.codes.append(&mut final_code);
        Ok(())
    }

    fn visit_init_stmt(&mut self, stmt: &StmtInit) -> CompileResultList<()> {
        let (init_res, mut init_code) = stmt.init.accept(self)?;
        let (var_type, slot, right_slot) = Self::pack_error(self.resolver.resolve_init_stmt(
            stmt,
            &init_res,
            self.context.in_loop,
        ))?;
        let mut final_code = Self::pack_error(self.compiler.compile_init_stmt(
            slot,
            right_slot,
            &mut init_code,
            &init_res,
            var_type,
        ))?;
        self.codes.append(&mut final_code);
        Ok(())
    }

    fn visit_assign_stmt(&mut self, stmt: &StmtAssign) -> CompileResultList<()> {
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

        Self::pack_error(
            self.resolver
                .resolve_assign_stmt(stmt, &vars_res, &right_res),
        )?;
        let mut final_code = Self::pack_error(self.compiler.compile_assign_stmt(
            &mut vars_code,
            &vars_res,
            &mut right_code,
            &right_res,
        ))?;
        self.context.in_assign = false;
        self.codes.append(&mut final_code);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> CompileResultList<()> {
        let mut errors = vec![];

        self.resolver.enter_scope();

        let mut context = self.context.save();
        self.compile_scope(&mut errors, &stmt.statements);

        // 检查末尾语句的返回情况。注意，此时 self.context 为子作用域的上下文
        if context.final_statement {
            if self.context.returned {
                context.returned = true;
            }
        }

        self.context.restore(context);

        let scope = self.resolver.leave_scope();

        // 单独的语句块不需要进行初始化一致性检查，所以直接初始化相关变量
        Self::scope_init_vars(&scope);

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> CompileResultList<()> {
        let (if_expr_res, mut if_expr_code) = stmt.if_branch.0.accept(self)?;
        let else_if_expr: Vec<_> = stmt.else_if_branch.iter().map(|(expr, _chunk)| expr.accept(self)).collect::<Result<_, _>>()?;
        let (else_if_expr_res, mut branch_expr_codes): (Vec<_>, Vec<_>) = else_if_expr.into_iter().unzip();

        self.resolver.resolve_if_stmt(stmt, &if_expr_res, &else_if_expr_res)?;

        let mut errors = vec![];
        let mut jump_end_locations = vec![];

        let mut returned_list = vec![false; 1 + stmt.else_if_branch.len() + if let Some(_) = stmt.else_branch { 1 } else { 0 }];
        let mut has_returned = false;

        // if 分支
        self.codes.append(&mut if_expr_code);
        self.write_code(OpJumpFalsePop);
        let false_jump_location = self.codes.len(); // 待会回填地址
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        self.resolver.enter_scope();

        let if_chunk = if let Stmt::Block(temp) = &stmt.if_branch.1 {
            temp
        } else {
            unreachable!("Not a block statement.")
        };
        let context = self.context.save();
        self.compile_scope(&mut errors, &if_chunk.statements);

        // 检查返回，若是最后一个语句，所有分支都要返回
        if context.final_statement {
            returned_list[0] = self.context.returned;
            has_returned = self.context.returned;
        }

        self.context.restore(context);
        let compare_scope = self.resolver.leave_scope();

        self.write_code(OpJump); // 跳转结尾
        jump_end_locations.push(self.codes.len());
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        let false_jump_dis = self.codes.len() - false_jump_location - 4; // 算上地址
        self.codes[false_jump_location..(false_jump_location + 4)]
            .copy_from_slice(&(false_jump_dis as u32).to_le_bytes());

        // else if 分支
        for (i, (condition_code, (_chunk_expr, chunk_block))) in branch_expr_codes.iter_mut().zip(stmt.else_if_branch.iter()).enumerate() {
            self.codes.append(condition_code);
            self.write_code(OpJumpFalsePop);
            let false_jump_location = self.codes.len();
            self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

            self.resolver.enter_scope();

            let chunk = if let Stmt::Block(temp) = chunk_block {
                temp
            } else {
                unreachable!("Not a block statement.")
            };
            let context = self.context.save();
            self.compile_scope(&mut errors, &chunk.statements);

            if context.final_statement {
                if self.context.returned {
                    has_returned = true;
                    returned_list[i + 1] = true;
                }
            }
            self.context.restore(context);
            let this_scope = self.resolver.leave_scope();

            if !Self::scopes_same_inits(&compare_scope, &this_scope) {
                errors.push(CompileError::new(
                    &chunk.pos,
                    "All code branches must initialize variables in the same way.".to_string(),
                ));
            }

            // 小优化：若没有 else 分支，直接跳过这一步
            if let Some(_) = stmt.else_branch {
                self.write_code(OpJump);
                jump_end_locations.push(self.codes.len());
                self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);
            }

            let false_jump_dis = self.codes.len() - false_jump_location - 4;
            self.codes[false_jump_location..(false_jump_location + 4)]
                .copy_from_slice(&(false_jump_dis as u32).to_le_bytes());
        }

        // else 分支
        if let Some(chunk_block) = &stmt.else_branch {
            self.resolver.enter_scope();

            let chunk = if let Stmt::Block(temp) = chunk_block {
                temp
            } else {
                unreachable!("Not a block statement.")
            };
            let context = self.context.save();
            self.compile_scope(&mut errors, &chunk.statements);

            if context.final_statement {
                if self.context.returned {
                    has_returned = true;
                    let final_idx = returned_list.len() - 1;
                    returned_list[final_idx] = true;
                }
            }
            self.context.restore(context);
            let this_scope = self.resolver.leave_scope();

            if !Self::scopes_same_inits(&compare_scope, &this_scope) {
                errors.push(CompileError::new(
                    &chunk.pos,
                    "All code branches must initialize variables in the same way.".to_string(),
                ));
            }
        }

        // 填充结尾跳转
        for jump_end_location in jump_end_locations {
            let jump_end_dis = self.codes.len() - jump_end_location - 4;
            self.codes[jump_end_location..(jump_end_location + 4)]
                .copy_from_slice(&(jump_end_dis as u32).to_le_bytes());
        }

        Self::scope_init_vars(&compare_scope);

        // 检查返回统一性
        if self.context.final_statement && has_returned {
            let mut uniform = true;
            for (i, returned) in returned_list.iter().enumerate() {
                if !returned {
                    uniform = false;
                    let pos = match i {
                        0 => {
                            let block = &stmt.if_branch.1;
                            stmt_get_pos!(block)
                        }
                        _ => {
                            if let Some(else_branch) = &stmt.else_branch {
                                if i == returned_list.len() - 1 {
                                    stmt_get_pos!(else_branch)
                                } else {
                                    let block = &stmt.else_if_branch[i - 1].1;
                                    stmt_get_pos!(block)
                                }
                            } else {
                                let block = &stmt.else_if_branch[i - 1].1;
                                stmt_get_pos!(block)
                            }
                        }
                    };
                    errors.push(CompileError::new(&pos, "The return of the final if statement needs to be uniform across branches.".to_string()));
                }
            }
            self.context.returned = uniform;
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn visit_loop_stmt(&mut self, stmt: &StmtLoop) -> CompileResultList<()> {
        // 提前分配内存
        self.write_code(OpStackExtend);
        let alloc_location = self.codes.len();
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        let start_location = self.codes.len();

        // 编译主体
        let before_slot = self.resolver.now_slot;
        self.resolver.enter_scope();
        self.context.in_loop = true;
        self.loop_tags.push(stmt.tag.clone());

        let mut errors = vec![];

        let chunk = if let Stmt::Block(temp) = &stmt.chunk {
            temp
        } else {
            unreachable!("Not a block statement.")
        };
        let context = self.context.save();
        self.compile_scope(&mut errors, &chunk.statements);

        let after_slot = self.resolver.now_slot;
        self.context.restore(context);
        self.resolver.leave_scope();
        let this_tag = self.loop_tags.pop().unwrap();
        self.context.in_loop = !self.loop_tags.is_empty();

        let jump_back = (self.codes.len() - start_location) as i32;
        self.write_code(OpJump);
        self.write_arg_dword((-jump_back - 5).to_le_bytes());

        // 补充跳出循环偏移地址
        for patch in &self.break_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_end = self.codes.len() as u32 - patch.patch_pos - 4; // 注意参数的 4 个字节
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)]
                    .copy_from_slice(&jump_end.to_le_bytes());
            }
        }
        self.break_patches
            .retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 补充继续循环偏移地址
        for patch in &self.continue_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_start = -(patch.patch_pos as i32 + 4 - start_location as i32);
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)]
                    .copy_from_slice(&jump_start.to_le_bytes());
            }
        }
        self.continue_patches
            .retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 分配与释放循环空间
        let memory_used = after_slot - before_slot;

        self.write_code(OpStackShrink);
        self.write_arg_dword((memory_used as u32).to_le_bytes());
        self.codes[alloc_location..(alloc_location + 4)]
            .copy_from_slice(&(memory_used as u32).to_le_bytes());

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> CompileResultList<()> {
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

        // 编译主体
        let before_slot = self.resolver.now_slot; // 用于计算循环占用的空间
        self.resolver.enter_scope();
        self.context.in_loop = true;
        self.loop_tags.push(stmt.tag.clone());

        let mut errors = vec![];

        let chunk = if let Stmt::Block(temp) = &stmt.chunk {
            temp
        } else {
            unreachable!("Not a block statement.")
        };
        let context = self.context.save();
        self.compile_scope(&mut errors, &chunk.statements);

        let after_slot = self.resolver.now_slot;
        self.context.restore(context);
        self.resolver.leave_scope();
        let this_tag = self.loop_tags.pop().unwrap();
        self.context.in_loop = !self.loop_tags.is_empty(); // 可能有嵌套的循环

        let jump_back = (self.codes.len() - start_location) as i32;
        self.write_code(OpJump);
        self.write_arg_dword((-jump_back - 5).to_le_bytes()); // 自己还有 5 字节！

        // 回填条件偏移地址
        let jump_end = self.codes.len() - jump_location - 4;
        self.codes[jump_location..(jump_location + 4)]
            .copy_from_slice(&(jump_end as u32).to_le_bytes());

        // 补充跳出循环偏移地址
        for patch in &self.break_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_end = self.codes.len() as u32 - patch.patch_pos - 4; // 注意参数的 4 个字节
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)]
                    .copy_from_slice(&jump_end.to_le_bytes());
            }
        }
        self.break_patches
            .retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 补充继续循环偏移地址
        for patch in &self.continue_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_start = -(patch.patch_pos as i32 + 4 - start_location as i32);
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)]
                    .copy_from_slice(&jump_start.to_le_bytes());
            }
        }
        self.continue_patches
            .retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 分配与释放循环空间
        let memory_used = after_slot - before_slot;

        self.write_code(OpStackShrink);
        self.write_arg_dword((memory_used as u32).to_le_bytes());
        self.codes[alloc_location..(alloc_location + 4)]
            .copy_from_slice(&(memory_used as u32).to_le_bytes());

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn visit_for_stmt(&mut self, stmt: &StmtFor) -> CompileResultList<()> {
        self.resolver.enter_scope(); // 保护作用域
        let protect_scope_before_slot = self.resolver.now_slot; // 保护作用域的偏移量
        self.resolver.predefine(slice::from_ref(&stmt.init))?;

        stmt.init.accept(self)?; // 编译初始化语句

        // 注意顺序
        let (condition_res, mut condition_code) = stmt.condition.accept(self)?;

        Self::pack_error(self.resolver.resolve_for_stmt(stmt, &condition_res))?;

        // 提前分配内存
        self.write_code(OpStackExtend);
        let alloc_location = self.codes.len();
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        let start_location = self.codes.len(); // 这才是循环开始

        // 写入表达式
        self.codes.append(&mut condition_code);
        self.write_code(OpJumpFalsePop);
        let jump_location = self.codes.len();
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        // 编译主体
        let before_slot = self.resolver.now_slot; // 用于计算循环占用的空间
        self.resolver.enter_scope();
        self.context.in_loop = true;
        self.loop_tags.push(stmt.tag.clone());

        let mut errors = vec![];

        let chunk = if let Stmt::Block(temp) = &stmt.chunk {
            temp
        } else {
            unreachable!("Not a block statement.")
        };
        let context = self.context.save();
        self.compile_scope(&mut errors, &chunk.statements);

        let after_slot = self.resolver.now_slot;
        self.context.restore(context);
        self.resolver.leave_scope();
        let this_tag = self.loop_tags.pop().unwrap();
        self.context.in_loop = !self.loop_tags.is_empty(); // 可能有嵌套的循环

        let update_location = self.codes.len();

        // 更新语句
        stmt.update.accept(self)?;

        let jump_back = (self.codes.len() - start_location) as i32;
        self.write_code(OpJump);
        self.write_arg_dword((-jump_back - 5).to_le_bytes()); // 自己还有 5 字节！

        // 回填条件偏移地址
        let jump_end = self.codes.len() - jump_location - 4;
        self.codes[jump_location..(jump_location + 4)]
            .copy_from_slice(&(jump_end as u32).to_le_bytes());

        // 补充跳出循环偏移地址
        for patch in &self.break_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_end = self.codes.len() as u32 - patch.patch_pos - 4; // 注意参数的 4 个字节
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)]
                    .copy_from_slice(&jump_end.to_le_bytes());
            }
        }
        self.break_patches
            .retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 补充继续循环偏移地址
        for patch in &self.continue_patches {
            if patch.loop_tag == this_tag || matches!(patch.loop_tag, None) {
                let jump_start = update_location as u32 - patch.patch_pos - 4;
                self.codes[(patch.patch_pos as usize)..(patch.patch_pos as usize + 4)]
                    .copy_from_slice(&jump_start.to_le_bytes());
            }
        }
        self.continue_patches
            .retain(|patch| (patch.loop_tag != this_tag) && !matches!(patch.loop_tag, None));

        // 分配与释放循环空间
        let memory_used = after_slot - before_slot;
        let protect_scope_after_slot = self.resolver.now_slot;
        let protect_scope_memory_used = protect_scope_after_slot - protect_scope_before_slot; // 保护作用于使用的空间

        self.write_code(OpStackShrink);
        self.write_arg_dword(((memory_used + protect_scope_memory_used) as u32).to_le_bytes());
        self.codes[alloc_location..(alloc_location + 4)]
            .copy_from_slice(&(memory_used as u32).to_le_bytes());

        self.resolver.leave_scope(); // 保护作用域

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn visit_break_stmt(&mut self, stmt: &StmtBreak) -> CompileResultList<()> {
        // 直接在这里进行分析并编译
        if !self.context.in_loop {
            return Err(vec![CompileError::new(
                &stmt.pos,
                "Cannot use 'break' outside a loop.".to_string(),
            )]);
        }

        Self::pack_error(self.check_tag(&stmt.tag, &stmt.pos))?;

        self.write_code(OpJump);
        let break_location = self.codes.len() as u32;
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        self.break_patches
            .push(BreakPatch::new(stmt.tag.clone(), break_location));

        Ok(())
    }

    fn visit_continue_stmt(&mut self, stmt: &StmtContinue) -> CompileResultList<()> {
        // 直接在这里分析并编译
        if !self.context.in_loop {
            return Err(vec![CompileError::new(
                &stmt.pos,
                "Cannot use 'continue' outside a loop.".to_string(),
            )]);
        }

        Self::pack_error(self.check_tag(&stmt.tag, &stmt.pos))?;

        self.write_code(OpJump);
        let continue_location = self.codes.len() as u32;
        self.write_arg_dword([0x00, 0x00, 0x00, 0x00]);

        self.continue_patches
            .push(ContinuePatch::new(stmt.tag.clone(), continue_location));

        Ok(())
    }

    fn visit_func_stmt(&mut self, stmt: &StmtFunc) -> CompileResultList<()> {
        Err(vec![CompileError::new(
            &stmt.pos,
            "Cannot use 'func' inside a function: Closure hasn't been implemented yet.".to_string(),
        )])
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> CompileResultList<()> {
        if let Some(expr) = &stmt.expr {
            let (expr_res, mut expr_code) = expr.accept(self)?;
            if &expr_res.res_type == self.current_function.get_return_type() {
                self.codes.append(&mut expr_code);
                self.write_code(match self.current_function.get_return_type().get_size() {
                    DataSize::Zero => OpReturnUnit,
                    DataSize::Byte => OpReturnByte,
                    DataSize::Word => OpReturnWord,
                    DataSize::Dword => OpReturnDword,
                    DataSize::Qword => OpReturnQword,
                    DataSize::Oword => OpReturnOword,
                });
                self.context.returned = true;
                Ok(())
            } else {
                Err(vec![CompileError::new(&stmt.pos, "Return type mismatched.".to_string())])
            }
        } else {
            if self.current_function.get_return_type() == &ValueType::Unit {
                self.write_code(OpReturnUnit);
                self.context.returned = true;
                Ok(())
            } else {
                Err(vec![CompileError::new(&stmt.pos, "Return type mismatched.".to_string())])
            }
        }
    }
}
