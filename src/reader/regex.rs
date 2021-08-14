use lazy_static::lazy_static;
use regex::Regex;

pub fn match_todo(text: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*)TODO[: ]? (.*)$").unwrap();
    }

    match RE.captures(text) {
        Some(cap) => cap.get(2).map(|todo| todo.as_str()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::match_todo;

    #[test]
    fn with_space() {
        assert_eq!(
            match_todo("// TODO rewrite everything").unwrap(),
            "rewrite everything"
        );
    }

    #[test]
    fn with_colon() {
        assert_eq!(
            match_todo("// TODO: rewrite everything").unwrap(),
            "rewrite everything"
        );
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(match_todo("// todo: rewrite everything").is_none(), true);
    }
}
