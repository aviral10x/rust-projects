use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Enter the first number:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let first_number: f64 = input.trim().parse().expect("Please type a number!");

        input.clear();
        println!("Enter the operator (+, -, *, /):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let operator = input.trim().to_string();  // Create a new owned string

        input.clear();
        println!("Enter the second number:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let second_number: f64 = input.trim().parse().expect("Please type a number!");

        let result = match operator.as_str() {  // Use as_str() to convert String back to &str
            "+" => first_number + second_number,
            "-" => first_number - second_number,
            "*" => first_number * second_number,
            "/" => first_number / second_number,
            _ => {
                println!("Invalid operator!");
                continue;
            }
        };

        println!("The result is: {}", result);
    }
}
