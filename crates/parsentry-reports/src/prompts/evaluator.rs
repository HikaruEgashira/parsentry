use parsentry_i18n::Language;

pub fn get_evaluator_prompt_template(lang: &Language) -> &'static str {
    parsentry_i18n::get_evaluator_prompt_template(lang)
}
