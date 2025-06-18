pub mod en;
pub mod ja;

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
}

impl LanguageConfig {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn get_message(&self, key: &str) -> &str {
        let messages = get_messages(&self.language);
        messages.get(key).unwrap_or(&"Message not found")
    }

    pub fn get_analysis_prompt(&self) -> &str {
        match self.language {
            Language::Japanese => "必ず日本語で応答してください",
            Language::English => "Please respond in English",
        }
    }

    pub fn get_response_language_instruction(&self) -> &str {
        get_response_language_instruction(&self.language)
    }
}

pub fn get_messages(lang: &Language) -> HashMap<&'static str, &'static str> {
    match lang {
        Language::Japanese => ja::get_messages(),
        Language::English => en::get_messages(),
    }
}

pub fn get_sys_prompt_template(lang: &Language) -> &'static str {
    match lang {
        Language::Japanese => ja::SYS_PROMPT_TEMPLATE,
        Language::English => en::SYS_PROMPT_TEMPLATE,
    }
}

pub fn get_initial_analysis_prompt_template(lang: &Language) -> &'static str {
    match lang {
        Language::Japanese => ja::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        Language::English => en::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
    }
}

pub fn get_analysis_approach_template(lang: &Language) -> &'static str {
    match lang {
        Language::Japanese => ja::ANALYSIS_APPROACH_TEMPLATE,
        Language::English => en::ANALYSIS_APPROACH_TEMPLATE,
    }
}

pub fn get_guidelines_template(lang: &Language) -> &'static str {
    match lang {
        Language::Japanese => ja::GUIDELINES_TEMPLATE,
        Language::English => en::GUIDELINES_TEMPLATE,
    }
}

pub fn get_evaluator_prompt_template(lang: &Language) -> &'static str {
    match lang {
        Language::Japanese => ja::EVALUATOR_PROMPT_TEMPLATE,
        Language::English => en::EVALUATOR_PROMPT_TEMPLATE,
    }
}

pub fn get_response_language_instruction(lang: &Language) -> &'static str {
    match lang {
        Language::Japanese => ja::RESPONSE_LANGUAGE_INSTRUCTION,
        Language::English => en::RESPONSE_LANGUAGE_INSTRUCTION,
    }
}
