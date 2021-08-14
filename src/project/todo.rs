use std::fmt;

pub struct Todo {
    id: Option<u32>,
    file_path: String,
    line: u32,
    content: String,
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
            "id             - {:?} \nfile_path      - {} \nline           - {} \ncontent        - {}",
            self.id, self.file_path, self.line, self.content
        )
    }
}
