use std::io;
fn main() {
    //instrction
    'cal: loop {
        println!("Hello, input what you want to calculate e.g 2 + 3, or enter q to break");
        //getting user input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("couldn't read line");
        // println!("{user_input}");
        if user_input.trim().contains("q") {
            break 'cal;
        }
        let tokens = tokenized_input(&user_input);
        let result = evaluate(&tokens);
        match result {
            Some(Ok(value)) => println!("Result: {}", value),
            Some(Err(err)) => println!("Error: {}", err),
            None => println!("Error: Invalid Expression"),
        }
    }
}

fn tokenized_input(user_input: &str) -> Vec<String> {
    user_input.split_whitespace().map(str::to_string).collect()
}
fn evaluate(tokens: &[String]) -> Option<Result<f64, String>> {
    let mut input = vec![];
    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            input.push(num);
            println!("{input:?}");
        } else if let Ok(operation) = token.parse::<char>() {
            if input.len() < 2 {
                return Some(Err("not enough oprands".to_string()));
            }
            let num2 = input.pop().unwrap();
            let num1 = input.pop().unwrap();
            let result = match operation {
                '+' => num1 + num2,
                '-' => num1 - num2,
                '*' => num1 * num2,
                '/' => {
                    if num2 == 0.0 {
                        return Some(Err("cannot divide by zero".to_string()));
                    }
                    num1 / num2
                }
                _ => return Some(Err("invalid operator".to_string())),
            };
            input.push(result);
        } else {
            return Some(Err("invalid token".to_string()));
        }
    }
    // if input.len() != 1 {
    //     return Some(Err("Invalid expression".to_string()));
    // }
    Some(Ok(input[0]))
}
