use {
    crate::core::{delta::Operation, Delta, Position, Size},
    std::{iter, mem, ops::AddAssign},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Text {
    lines: Vec<Vec<char>>,
}

impl Text {
    pub fn new() -> Text {
        Text::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len().is_zero()
    }

    pub fn len(&self) -> Size {
        Size {
            line: self.lines.len() - 1,
            column: self.lines.last().unwrap().len(),
        }
    }

    pub fn as_lines(&self) -> &[Vec<char>] {
        &self.lines
    }

    pub fn take(&mut self, len: Size) -> Text {
        let mut lines = self.lines.drain(..len.line).collect::<Vec<_>>();
        lines.push(
            self.lines
                .first_mut()
                .unwrap()
                .drain(..len.column)
                .collect::<Vec<_>>(),
        );
        Text { lines }
    }

    pub fn skip(&mut self, len: Size) {
        self.lines.drain(..len.line);
        self.lines.first_mut().unwrap().drain(..len.column);
    }

    pub fn insert(&mut self, position: Position, mut text: Text) {
        if text.len().line == 0 {
            self.lines[position.line].splice(
                position.column..position.column,
                text.lines.first().unwrap().iter().cloned(),
            );
        } else {
            text.lines.first_mut().unwrap().splice(
                ..0,
                self.lines[position.line][..position.column].iter().cloned(),
            );
            text.lines.last_mut().unwrap().splice(
                self.lines[position.line].len()..,
                self.lines[position.line][position.column..].iter().cloned(),
            );
            self.lines
                .splice(position.line..position.line + 1, text.lines.into_iter());
        }
    }

    pub fn delete(&mut self, position: Position, count: Size) {
        if count.line == 0 {
            self.lines[position.line].splice(
                position.column..position.column + count.column,
                iter::empty(),
            );
        } else {
            let mut line = mem::replace(&mut self.lines[position.line], Vec::new());
            line.splice(
                position.column..,
                self.lines[position.line + count.line][count.column..]
                    .iter()
                    .cloned(),
            );
            self.lines.splice(
                position.line..position.line + count.line + 1,
                iter::once(line),
            );
        }
    }

    pub fn apply_delta(&mut self, delta: Delta) {
        let mut position = Position::origin();
        for operation in delta {
            match operation {
                Operation::Retain(len) => position += len,
                Operation::Insert(text) => {
                    let len = text.len();
                    self.insert(position, text);
                    position += len;
                }
                Operation::Delete(len) => self.delete(position, len),
            }
        }
    }
}

impl AddAssign for Text {
    fn add_assign(&mut self, other: Text) {
        self.lines
            .last_mut()
            .unwrap()
            .extend(other.lines.first().unwrap());
        self.lines.extend(other.lines.into_iter().skip(1))
    }
}

impl Default for Text {
    fn default() -> Text {
        Text {
            lines: vec![Vec::default()],
        }
    }
}

impl From<Vec<Vec<char>>> for Text {
    fn from(lines: Vec<Vec<char>>) -> Text {
        Text { lines }
    }
}
