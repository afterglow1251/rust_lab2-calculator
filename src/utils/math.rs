pub(crate) fn apply_operator(op: char, a: f64, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err("Division by zero".into())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("Unknown operator".into()),
    }
}
