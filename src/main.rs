mod cli;
mod calculator;

pub use cli::cli_readline::{readline_float, readline_string};
pub use calculator::{Calculator, Op};

fn main() {
    let num1: f32 = readline_float(String::from("Enter first number: "));
    let op: String = readline_string(String::from("Enter the operator(+, -, x, /, %): "));
    let num2: f32 = readline_float(String::from("Enter second number: "));

    match op.as_str() {
       "+"  => {
            let addition = Calculator { operand1: num1, operand2: num2, op: Op::Add };
            let result = addition.calc();
            println!("{} + {} = {}", num1, num2, result);
        }
       _ => println!("Invalid input"),
    }
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
