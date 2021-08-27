use std::fmt;

pub struct Todo {
    pub id: Option<u32>,
    pub file_path: String,
    pub line: u32,
    pub content: String,
}

impl Todo {
    pub fn from(id: Option<u32>, file_path: String, line: u32, content: String) -> Self {
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
