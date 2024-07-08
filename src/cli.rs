pub mod cli_readline {
    pub fn readline_float(message: String) -> f32 {
        let prompt: f32 = rprompt::prompt_reply(format!("{message}")).expect("Invalid Input").trim().parse().unwrap();
        prompt
    }

    pub fn readline_string(message: String) -> String {
        let prompt: String = rprompt::prompt_reply(format!("{message}")).unwrap();
        prompt
    }
}
