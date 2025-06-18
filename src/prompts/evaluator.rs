use crate::locales::Language;
use crate::locales;

pub fn get_evaluator_prompt_template(lang: &Language) -> &'static str {
    locales::get_evaluator_prompt_template(lang)
}
