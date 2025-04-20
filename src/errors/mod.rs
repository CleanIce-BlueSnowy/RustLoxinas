//! 错误处理模块

pub mod error_types;

use crate::position::Position;
use error_types::CompileError;
use error_types::LexicalError;
use error_types::SyntaxError;

/// 错误列表
#[must_use]
pub enum ErrorList<'a> {
    LexicalErrors(&'a [LexicalError]),
    SyntaxErrors(&'a [SyntaxError]),
    CompileErrors(&'a [CompileError]),
}

/// 打印错误列表
#[must_use]
pub fn print_all_errors(lines: &[&str], errors: ErrorList) -> String {
    let mut result = String::new();
    match errors {
        ErrorList::LexicalErrors(errors) => {
            for error in errors {
                result.push_str(&print_error(
                    "Lexical Error",
                    lines,
                    &error.message,
                    &error.pos,
                ));
            }
        }
        ErrorList::SyntaxErrors(errors) => {
            for error in errors {
                result.push_str(&print_error(
                    "Syntax Error",
                    lines,
                    &error.message,
                    &error.pos,
                ));
            }
        }
        ErrorList::CompileErrors(errors) => {
            for error in errors {
                result.push_str(&print_error(
                    "Compile Error",
                    lines,
                    &error.message,
                    &error.pos,
                ));
            }
        }
    }
    return result;
}

/** 打印错误（返回字符串）

接受错误类型、代码行、错误位置

错误格式（单行）：

```text
<Error Type>: line ? at ?-?: <Error Message>
  |> This is the code and here leads an error
                          ^^^^
```

错误格式（两行）：
```text
<Error Type>: from (line ? at ?) to (line ? at ?): <Error Message>
  |> This is the first line and here begins the error
                                ^^^^^^^^^^^^^^^^^^^^^
  |> This is the last line and here ends the error
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

错误格式（多行）：
```text
<Error Type>: from (line ? at ?) to (line ? at ?): <Error Message>
  |> This is the first line and here begins the error
                                ^^^^^^^^^^^^^^^^^^^^^
  |> ...
  |> This is the last line and here ends the error
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
 */
#[must_use]
pub fn print_error(error_type: &str, lines: &[&str], message: &str, pos: &Position) -> String {
    let mut res = if pos.start_line == pos.end_line {
        // 根据是否在同一行给出不同的输出格式
        format!(
            "{}: line {} at {}-{}: {}\n",
            error_type,
            pos.start_line,
            pos.start_idx + 1,
            pos.end_idx,
            message
        )
    } else {
        format!(
            "{}: from (line {} at {}) to (line {} at {}): {}\n",
            error_type,
            pos.start_line,
            pos.start_idx + 1,
            pos.end_line,
            pos.end_idx + 1,
            message
        )
    };

    let line = lines[pos.start_line - 1]; // 起始行
    res.push_str(&format!("  |> {}\n     ", line));
    let end_idx = if pos.start_line == pos.end_line {
        // 确认起始行位置提示终止位置
        pos.end_idx
    } else {
        let chars: Vec<char> = line.chars().collect();
        chars.len() - 1
    };

    // 打印起始行位置提示
    for _i in 0..pos.start_idx {
        res.push(' ');
    }
    for _i in pos.start_idx..end_idx {
        res.push('^');
    }
    res.push('\n');

    // 若错误不在一行以内
    if pos.start_line != pos.end_line {
        if pos.end_line - pos.start_line > 1 {
            // 错误行数大于 2 行，则省略中间行
            res.push_str("  |> ...\n");
        }
        let line = lines[pos.end_line - 1]; // 终止行
        res.push_str(&format!("  |> {}\n     ", line));
        for _i in 0..pos.end_idx {
            // 打印终止行位置提示
            res.push('^');
        }
        res.push('\n');
    }

    return res;
}

/** 打印运行时错误

功能不完全，因为字节码符号表尚未完成

错误格式：

```text
Runtime Error: <Error Message>
```
 */
#[must_use]
pub fn print_runtime_error(msg: &str) -> String {
    format!("Runtime Error: {}", msg)
}
