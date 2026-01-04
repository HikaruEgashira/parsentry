#[cfg(test)]
mod language_filter_tests {
    use parsentry_core::Language;
    use std::collections::HashSet;
    use std::str::FromStr;

    #[test]
    fn test_csv_parsing() {
        let filter_str = "python,rust,javascript";
        let languages: HashSet<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();

        assert_eq!(languages.len(), 3);
        assert!(languages.contains(&Language::Python));
        assert!(languages.contains(&Language::Rust));
        assert!(languages.contains(&Language::JavaScript));
    }

    #[test]
    fn test_whitespace_handling() {
        let filter_str = " python , rust , javascript ";
        let languages: Vec<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();

        assert_eq!(languages.len(), 3);
    }

    #[test]
    fn test_alias_support() {
        let filter_str = "py,js,rs";
        let languages: Vec<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();

        assert_eq!(languages.len(), 3);
        assert!(languages.contains(&Language::Python));
        assert!(languages.contains(&Language::JavaScript));
        assert!(languages.contains(&Language::Rust));
    }

    #[test]
    fn test_case_insensitivity() {
        let filter_str = "PYTHON,Rust,javaScript";
        let languages: HashSet<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();

        assert_eq!(languages.len(), 3);
        assert!(languages.contains(&Language::Python));
        assert!(languages.contains(&Language::Rust));
        assert!(languages.contains(&Language::JavaScript));
    }

    #[test]
    fn test_invalid_language_filtering() {
        let filter_str = "python,invalid_lang,rust";
        let languages: Vec<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();

        // Only valid languages should be included
        assert_eq!(languages.len(), 2);
        assert!(languages.contains(&Language::Python));
        assert!(languages.contains(&Language::Rust));
    }

    #[test]
    fn test_empty_filter() {
        let filter_str = "";
        let languages: Vec<Language> = filter_str
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Language::from_str(trimmed).ok()
                }
            })
            .collect();

        assert_eq!(languages.len(), 0);
    }

    #[test]
    fn test_all_supported_languages() {
        let filter_str = "python,javascript,rust,typescript,java,go,ruby,c,cpp,terraform,cloudformation,kubernetes,yaml,bash,shell,php";
        let languages: HashSet<Language> = filter_str
            .split(',')
            .filter_map(|s| Language::from_str(s.trim()).ok())
            .collect();

        // Should parse all supported languages
        assert_eq!(languages.len(), 16);
    }
}
