mod cli;

pub use cli::cli_readline::{readline_float, readline_string};

fn main() {
    let num1: f32 = readline_float(String::from("Enter first number: "));
    println!("{}", num1);
    let operation: String = readline_string(String::from("Enter the operator(+, -, x, /, %): "));
    println!("{}", operation);
    let num2: f32 = readline_float(String::from("Enter second number: "));
    println!("{}", num2);
}
/*
fn readline_float(message: String) -> f32 {
    let prompt: f32 = rprompt::prompt_reply(format!("{message}")).expect("Sorry, enter a valid number").parse().unwrap();
    prompt
}

fn readline_string(message: String) -> String {
    let prompt: String = rprompt::prompt_reply(format!("{message}")).unwrap();
    prompt
}
*/
