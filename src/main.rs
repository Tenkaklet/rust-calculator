use std::env::{args, Args};

fn main() {
    let mut args: Args = args(); // retrieve args from CLI
    
    let first = args.nth(1).unwrap(); // Get the first value
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    
    let first_number = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse().unwrap();
    
    let result = operate(operator, first_number, second_number);
    
    let compute = output(first_number, operator, second_number, result);
    
    println!("{:?}", compute);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used.")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{first_number} {operator} {second_number} = {result}")
}