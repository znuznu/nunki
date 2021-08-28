use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

// pub fn get_untracked_pattern<'a>(keywords: &'a [&'a str]) -> &'a str {
//     let joined_keywords = keywords.join("|");

//     return format!("^(.*)[{}][: ]? (.*)$", joined_keywords);
// }

pub fn extract_untracked_todo_content<'a>(
    line: &'a str,
    // keywords_pattern: &str,
) -> Option<&'a str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*)TODO[: ]? (.*)(\n)?$").unwrap();
    }

    match RE.captures(line) {
        Some(cap) => cap.get(2).map(|todo| todo.as_str()),
        None => None,
    }
}

/// Add the provided id to the untracked line
pub fn replace_untracked_todo(line: &str, id: u32) -> Cow<'_, str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<prefix>.*)(?P<keyword>TODO)(?P<end>[: ]? .*(\n)?)$").unwrap();
    }

    let keyword_with_id = format!("$prefix$keyword(#{})$end", id);

    RE.replace_all(line, keyword_with_id)
}

#[cfg(test)]
mod tests {
    use super::{extract_untracked_todo_content, replace_untracked_todo};

    #[test]
    fn with_space() {
        assert_eq!(
            extract_untracked_todo_content("// TODO rewrite everything").unwrap(),
            "rewrite everything"
        );
    }

    #[test]
    fn with_space_and_newline() {
        assert_eq!(
            extract_untracked_todo_content("// TODO rewrite everything\n").unwrap(),
            "rewrite everything"
        );
    }

    #[test]
    fn with_colon() {
        assert_eq!(
            extract_untracked_todo_content("// TODO: rewrite everything").unwrap(),
            "rewrite everything"
        );
    }

    #[test]
    fn with_colon_and_newline() {
        assert_eq!(
            extract_untracked_todo_content("// TODO: rewrite everything\n").unwrap(),
            "rewrite everything"
        );
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(
            extract_untracked_todo_content("// todo: rewrite everything").is_none(),
            true
        );
    }

    #[test]
    fn replace_untracked_with_colon() {
        assert_eq!(
            replace_untracked_todo("// TODO: write some nasm for fun", 42),
            "// TODO(#42): write some nasm for fun"
        );
    }

    #[test]
    fn replace_untracked_without_colon() {
        assert_eq!(
            replace_untracked_todo("// TODO write some nasm for fun", 42),
            "// TODO(#42) write some nasm for fun"
        );
    }

    #[test]
    fn replace_tracked_without_colon() {
        assert_eq!(
            replace_untracked_todo("// TODO(#42) write some nasm for fun", 42),
            "// TODO(#42) write some nasm for fun"
        );
    }

    #[test]
    fn replace_tracked_without_colon_but_newline() {
        assert_eq!(
            replace_untracked_todo("// TODO(#42) write some nasm for fun\n", 42),
            "// TODO(#42) write some nasm for fun\n"
        );
    }

    #[test]
    fn replace_tracked_with_colon() {
        assert_eq!(
            replace_untracked_todo("// TODO(#42): write some nasm for fun", 42),
            "// TODO(#42): write some nasm for fun"
        );
    }

    #[test]
    fn replace_tracked_with_colon_and_newline() {
        assert_eq!(
            replace_untracked_todo("// TODO(#42): write some nasm for fun\n", 42),
            "// TODO(#42): write some nasm for fun\n"
        );
    }
}
