pub(crate) fn apply_operation(left: f64, right: f64, operator: char) -> Result<f64, String> {
    match operator {
        '+' => Ok(left + right),
        '-' => Ok(left - right),
        '*' => Ok(left * right),
        '/' => {
            if right == 0.0 {
                Err("Error: division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err("Unknown operator".to_string()),
    }
}