use std::ops::{Add, AddAssign};

#[derive(Clone, Debug)]
pub struct Position {
    raw_position: usize,
    file: String,
}

impl Position {
    pub fn new(file: String, index: usize) -> Self {
        Self {
            raw_position: index,
            file,
        }
    }

    // Returns the current index in the input string.
    pub fn index(&self) -> usize {
        self.raw_position
    }

    pub fn advance(&mut self, steps: usize) {
        self.raw_position += steps;
    }

    pub fn line(&self, input: &str) -> usize {
        input[..self.raw_position].lines().count()
    }

    pub fn column(&self, input: &str) -> usize {
        let last_newline = input[..self.raw_position].rfind('\n');
        match last_newline {
            Some(pos) => self.raw_position - pos,
            None => self.raw_position + 1,
        }
    }

    pub fn format(&self, input: &str) -> String {
        let line_num = self.line(input);
        let column_num = self.column(input);
        format!(
            "line {}, column {} (in file '{}')",
            line_num, column_num, self.file
        )
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.raw_position == other.raw_position && self.file == other.file
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.file != other.file {
            None
        } else {
            self.raw_position.partial_cmp(&other.raw_position)
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.file != other.file {
            panic!("Cannot add positions from different files");
        }
        Self {
            raw_position: self.raw_position + other.raw_position,
            file: self.file,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        if self.file != other.file {
            panic!("Cannot add positions from different files");
        }
        self.raw_position += other.raw_position;
    }
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        if start.file != end.file {
            panic!("Start and end positions must be from the same file");
        }
        Self { start, end }
    }
}
