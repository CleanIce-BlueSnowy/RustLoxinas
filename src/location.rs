#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn create(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: Position {
                line: start_line,
                col: start_col,
            },
            end: Position {
                line: end_line,
                col: end_col,
            },
        }
    }

    pub fn bind(start_loc: Self, end_loc: Self) -> Self {
        Self {
            start: start_loc.start,
            end: end_loc.end,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {line}", line = self.start.line)?;
        if self.end.line != self.start.line {
            write!(f, "-{line}", line = self.end.line)?;
        }
        write!(f, " at {pos}", pos = self.start.col + 1)?;
        if self.end.col != self.start.col + 1 {
            write!(f, "-{pos}", pos = self.end.col + 1)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}
