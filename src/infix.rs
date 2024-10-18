use std::vec::Vec;
use crate::utils::math::apply_operator;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

/// Evaluates an infix mathematical expression and returns the result as a f64.
/// It tokenizes the input string, converts the tokens to Reverse Polish Notation (RPN),
/// and then evaluates the RPN expression.
pub(crate) fn evaluate_infix_expression(expr: &str) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    let rpn = shunting_yard(tokens)?;
    evaluate_rpn(rpn)
}

/// Flushes the number buffer into tokens if it's not empty.
/// Parses the buffered number and adds it as a Token::Number to the tokens vector.
fn flush_number_buffer(num_buffer: &mut String, tokens: &mut Vec<Token>) -> Result<(), String> {
    if !num_buffer.is_empty() {
        if let Ok(num) = num_buffer.parse::<f64>() {
            tokens.push(Token::Number(num));
            num_buffer.clear();
        } else {
            return Err("Failed to parse number.".into());
        }
    }
    Ok(())
}

/// Tokenizes the input expression string into a vector of Token enums.
/// Handles numbers, operators, and parentheses, and ensures the correct tokenization
/// based on the rules of mathematical expressions.
fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut num_buffer = String::new();
    let mut prev_token_is_operator = true;

    for c in expr.chars() {
        match c {
            '0'..='9' | '.' => {
                num_buffer.push(c);
                prev_token_is_operator = false;
            }
            '+' | '*' | '/' => {
                flush_number_buffer(&mut num_buffer, &mut tokens)?;
                tokens.push(Token::Operator(c));
                prev_token_is_operator = true;
            }
            '-' => {
                flush_number_buffer(&mut num_buffer, &mut tokens)?;

                if prev_token_is_operator {
                    num_buffer.push('-');
                } else {
                    tokens.push(Token::Operator('-'));
                    prev_token_is_operator = true;
                }
            }
            '(' => {
                flush_number_buffer(&mut num_buffer, &mut tokens)?;
                tokens.push(Token::LeftParen);
                prev_token_is_operator = true;
            }
            ')' => {
                flush_number_buffer(&mut num_buffer, &mut tokens)?;
                tokens.push(Token::RightParen);
                prev_token_is_operator = false;
            }
            ' ' => {
                flush_number_buffer(&mut num_buffer, &mut tokens)?;
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    flush_number_buffer(&mut num_buffer, &mut tokens)?;

    Ok(tokens)
}


/// Returns the precedence of the given operator as an unsigned 8-bit integer.
/// Operators with higher precedence are evaluated first.
fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

/// Converts the vector of tokens from infix notation to Reverse Polish Notation (RPN)
/// using the Shunting Yard algorithm. Handles operator precedence and parentheses.
fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(Token::Operator(top_op)) = operators.last() {
                    if precedence(*top_op) >= precedence(op) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(Token::Operator(op));
            }
            Token::LeftParen => operators.push(Token::LeftParen),
            Token::RightParen => {
                while let Some(op) = operators.pop() {
                    if op == Token::LeftParen {
                        break;
                    } else {
                        output.push(op);
                    }
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}

/// Evaluates the given vector of tokens in Reverse Polish Notation (RPN)
/// and returns the result as a f64.
fn evaluate_rpn(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err("Invalid expression".into());
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = apply_operator(op, left, right)?;
                stack.push(result);
            }
            _ => return Err("Invalid token".into()),
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Invalid expression".into())
    }
}

