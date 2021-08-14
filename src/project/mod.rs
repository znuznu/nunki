use todo::Todo;

pub mod todo;

/// The project behavior.
/// Matching - Search and notify about untracked todos, without affecting the source code
/// Tracking - Track the todos by creating an issue and adding an id to the todo before comitting code
enum Behavior {
    Matching,
    Tracking,
}

struct Project {
    // tracked_todos: Vec<Todo>,
// untracked_todos: Vec<Todo>,
}

impl Project {}
