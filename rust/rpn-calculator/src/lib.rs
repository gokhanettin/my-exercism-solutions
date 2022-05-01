#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            op => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                let z = match op {
                    CalculatorInput::Add => y + x,
                    CalculatorInput::Subtract => y - x,
                    CalculatorInput::Multiply => y * x,
                    CalculatorInput::Divide => y / x,
                    _ => unreachable!(),
                };
                stack.push(z);
            }
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
