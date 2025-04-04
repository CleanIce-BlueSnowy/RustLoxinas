//! 前端编译器辅助模块

use crate::front_compiler::FrontCompiler;

impl<'a> FrontCompiler<'a> {
    /// 打包错误到错误列表
    #[inline]
    pub fn pack_error<OkType, ErrType>(result: Result<OkType, ErrType>) -> Result<OkType, Vec<ErrType>> {
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => Err(vec![err]),
        }
    }
}