pub struct Calculator {
    pub operand1: f32,
    pub operand2: f32,
    pub op: Op,
}

pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
}

impl Calculator {
    pub fn calc(&self) -> f32 {
        match self.op {
            Op::Add => self.operand1 + self.operand2,
            Op::Subtract => self.operand1 - self.operand2,
            Op::Multiply => self.operand1 * self.operand2,
            Op::Divide => self.operand1 / self.operand2,
            Op::Modulus => self.operand1 % self.operand2,
        }
    }
}

pub fn addition(num1: f32, num2: f32, op: Op) {
    let new_calculator = Calculator { operand1: num1, operand2: num2, op: op };
    let result = new_calculator.calc();
    println!("{} + {} = {}", num1, num2, result);
}

pub fn subtraction(num1: f32, num2: f32, op: Op) {
    let new_calculator = Calculator { operand1: num1, operand2: num2, op: op };
    let result = new_calculator.calc();
    println!("{} - {} = {}", num1, num2, result);
}

pub fn multiplication(num1: f32, num2: f32, op: Op) {
    let new_calculator = Calculator { operand1: num1, operand2: num2, op: op };
    let result = new_calculator.calc();
    println!("{} x {} = {}", num1, num2, result);
}

pub fn division(num1: f32, num2: f32, op: Op) {
    if num2 == 0.0 {
        println!("You cannot divide by 0");
        return
    }
    let new_calculator = Calculator { operand1: num1, operand2: num2, op: op };
    let result = new_calculator.calc();
    println!("{} / {} = {}", num1, num2, result);
}

pub fn modulus(num1: f32, num2: f32, op: Op) {
    let new_calculator = Calculator { operand1: num1, operand2: num2, op: op };
    let result = new_calculator.calc();
    println!("{} % {} = {}", num1, num2, result);
}
