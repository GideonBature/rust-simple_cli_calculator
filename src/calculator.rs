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
