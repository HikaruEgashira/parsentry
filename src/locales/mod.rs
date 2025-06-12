pub mod en;
pub mod ja;

use crate::language::Language;
use std::collections::HashMap;

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
