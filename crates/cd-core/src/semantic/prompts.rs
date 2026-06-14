use crate::models::analysis::BlockType;

pub fn language_prompt(text: &str, target_lang: &str) -> String {
    format!(
        "Translate the following text to {target_lang}. \
        Return only the translation, nothing else. \
        Preserve formatting, line breaks and code terms as-is.\n\n\
        Text:\n{text}"
    )
}

pub fn code_explain_prompt(code: &str, lang_hint: Option<&str>) -> String {
    let lang_str = lang_hint.map(|l| format!(" ({l})")).unwrap_or_default();
    format!(
        "Explain this code{lang_str} concisely:\n\
        1. Purpose — what does it do?\n\
        2. How it works — key logic\n\
        3. Potential issues or improvements\n\n\
        Keep the answer brief and developer-focused.\n\n\
        Code:\n```\n{code}\n```"
    )
}

pub fn terminal_prompt(output: &str) -> String {
    format!(
        "Analyze this terminal output:\n\
        1. What happened?\n\
        2. Is there an error? If so, what caused it?\n\
        3. Recommended next steps.\n\n\
        Be concise and actionable.\n\n\
        Output:\n{output}"
    )
}

pub fn log_prompt(log: &str) -> String {
    format!(
        "Analyze these log entries:\n\
        1. Summary of events\n\
        2. Errors or warnings — what do they indicate?\n\
        3. Suggested fix or investigation path\n\n\
        Output:\n{log}"
    )
}

pub fn smart_prompt(text: &str, block_type: &BlockType, target_lang: &str) -> String {
    match block_type {
        BlockType::Code { lang_hint } => code_explain_prompt(text, lang_hint.as_deref()),
        BlockType::Terminal           => terminal_prompt(text),
        BlockType::Log                => log_prompt(text),
        _                             => language_prompt(text, target_lang),
    }
}
