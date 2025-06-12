use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Japanese,
    English,
}

impl Language {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ja" | "japanese" => Language::Japanese,
            "en" | "english" => Language::English,
            _ => Language::Japanese, // Default to Japanese
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Language::Japanese => "ja",
            Language::English => "en",
        }
    }
}

pub struct LanguageConfig {
    pub language: Language,
    messages: HashMap<&'static str, HashMap<Language, &'static str>>,
}

impl LanguageConfig {
    pub fn new(language: Language) -> Self {
        let mut messages = HashMap::new();
        
        // Add common messages
        messages.insert("error_clone_failed", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "クローン先ディレクトリの削除に失敗");
            m.insert(Language::English, "Failed to delete clone directory");
            m
        });
        
        messages.insert("cloning_repo", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "GitHubリポジトリをクローン中");
            m.insert(Language::English, "Cloning GitHub repository");
            m
        });
        
        messages.insert("analysis_target", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "解析対象");
            m.insert(Language::English, "Analysis target");
            m
        });
        
        messages.insert("context_collection_failed", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "コンテキスト収集に失敗");
            m.insert(Language::English, "Failed to collect context");
            m
        });
        
        messages.insert("analyzing_file", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "ファイルを解析中");
            m.insert(Language::English, "Analyzing file");
            m
        });
        
        messages.insert("analysis_completed", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "解析完了");
            m.insert(Language::English, "Analysis completed");
            m
        });
        
        messages.insert("error_directory_creation", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "ディレクトリの作成に失敗");
            m.insert(Language::English, "Failed to create directory");
            m
        });
        
        messages.insert("error_no_write_permission", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "書き込み権限がありません");
            m.insert(Language::English, "No write permission");
            m
        });
        
        messages.insert("error_test_file_deletion", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "テストファイルの削除に失敗");
            m.insert(Language::English, "Failed to delete test file");
            m
        });
        
        messages.insert("error_no_file_creation_permission", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "ファイル作成権限がありません");
            m.insert(Language::English, "No file creation permission");
            m
        });
        
        messages.insert("error_output_dir_check", {
            let mut m = HashMap::new();
            m.insert(Language::Japanese, "❌ 出力ディレクトリのチェックに失敗");
            m.insert(Language::English, "❌ Failed to check output directory");
            m
        });

        Self {
            language,
            messages,
        }
    }

    pub fn get_message(&self, key: &str) -> &str {
        self.messages
            .get(key)
            .and_then(|lang_map| lang_map.get(&self.language))
            .unwrap_or(&"Message not found")
    }

    pub fn get_analysis_prompt(&self) -> &str {
        match self.language {
            Language::Japanese => "必ず日本語で応答してください",
            Language::English => "Please respond in English",
        }
    }

    pub fn get_response_language_instruction(&self) -> &str {
        match self.language {
            Language::Japanese => "7. 必ず日本語で応答してください",
            Language::English => "7. Please respond in English",
        }
    }
}