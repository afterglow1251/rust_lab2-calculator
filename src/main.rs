use std::io;
mod infix;
mod polish;
mod utils;
mod constants;

use crate::utils::common::{evaluate_expression, check_exit_command, choose_mode, mode_to_string};
use crate::constants::{Mode, MODE_COMMON, MODE_POLISH, EXIT_COMMAND, MEMORY_FLAG};


fn main() {
    let mut mode: Option<Mode> = None; // Variable to store mode
    let mut last_result: Option<f64> = None; // Variable to store the last result in memory

    loop {
        // Select mode if not already selected
        if mode.is_none() {
            println!("Choose a mode:\n\
            - Common (type '{}')\n\
            - Polish notation (type '{}')\n\
            q to exit", MODE_COMMON, MODE_POLISH);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read string");
            let input = input.trim();

            // Check for output
            if check_exit_command(input) {
                return;
            }

            // Setting the new mode
            if let Some(new_mode) = choose_mode(input) {
                mode = Some(new_mode);
            } else {
                println!("Incorrect choice. Try again");
            }
        }

        // If the mode is still None, continue asking
        if mode.is_none() {
            continue;
        }

        println!("Now let's process expressions in mode: {}", mode_to_string(&mode));

        loop {
            println!("Enter an expression to calculate:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim();

            // Check for output
            if check_exit_command(input) {
                return;
            }

            // Handle the case when MEMORY_FLAG is entered to use the last result
            let expression = if input.contains(MEMORY_FLAG) {
                if let Some(result) = last_result {
                    // Replace "-R" with the last result
                    input.replace(MEMORY_FLAG, &result.to_string()) // Use the last result
                } else {
                    println!("No result available."); // If there is no result
                    continue;
                }
            } else {
                input.to_string() // Otherwise, use the entered value
            };

            // Check for mode change
            if let Some(new_mode) = choose_mode(&expression) {
                mode = Some(new_mode);
                println!("Mode changed to: {}", mode_to_string(&mode));
                continue; // Go to the request to enter an expression
            }

            // Evaluate the expression
            match evaluate_expression(&expression, mode.as_ref().unwrap()) {
                Ok(result) => {
                    last_result = Some(result); // Save the result for future use
                    println!("Result: {}", result);
                }
                Err(e) => println!("Error: {}", e), // Print an error if it occurs
            }
        }
    }
}
