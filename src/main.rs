
use std::io;

fn main() {
    loop {
        println!("Enter an expression:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "q" || input == "quit" {
            break;
        }

        let result = match input.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                let tokens: Vec<&str> = input.split(" ").collect();
                if tokens.len() != 3 {
                    println!("Invalid input, please enter a valid expression or 'q' to quit.");
                    continue;
                }

                let num1 = match tokens[0].parse::<f64>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a valid expression or 'q' to quit.");
                        continue;
                    }
                };

                let num2 = match tokens[2].parse::<f64>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a valid expression or 'q' to quit.");
                        continue;
                    }
                };

                match tokens[1] {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => num1 / num2,
                    _ => {
                        println!("Invalid input, please enter a valid expression or 'q' to quit.");
                        continue;
                    }
                }
            }
        };

        println!("Result: {}", result);
    }
}
