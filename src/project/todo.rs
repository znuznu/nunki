use std::fmt;

pub struct Todo {
    pub id: Option<usize>,
    pub file_path: String,
    pub line: usize,
    pub content: String,
}

impl Todo {
    pub fn from(id: Option<usize>, file_path: String, line: usize, content: String) -> Self {
        Todo {
            id,
            file_path,
            line,
            content,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{:?} - L{}\n{}",
            self.file_path, self.id, self.line, self.content
        )
    }
}
