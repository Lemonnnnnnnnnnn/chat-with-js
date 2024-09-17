use regex::Regex;


pub fn find_exports(content: &str) -> Vec<&str> {
    let re = Regex::new(r#"export\s+.*?['"](.+)['"];?"#).unwrap();
    re.captures_iter(content)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect()
}

pub fn find_imports(content: &str) -> Vec<&str> {
    let re = Regex::new(r#"import\s+.*?["'](.+?)["'];?"#).unwrap();
    re.captures_iter(content)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_exports() {
        let content = r#"
            export { foo, bar } from './a/b'
            export { baz } from './c/d'
        "#;
        let expected = vec!["./a/b", "./c/d"];
        assert_eq!(find_exports(content), expected);
    }

    #[test]
    fn test_find_imports() {
        let content = r#"
            import { foo } from './a/b'
            import { bar, baz } from './c/d'
        "#;
        let expected = vec!["./a/b", "./c/d"];
        assert_eq!(find_imports(content), expected);
    }

    #[test]
    fn test_cannot_find_imports() {
        let content = r#"
        "#;
        let expected: Vec<&str> = vec![];
        assert_eq!(find_imports(content), expected);
    } 
}



