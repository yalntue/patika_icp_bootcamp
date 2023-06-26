enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let first_number: f64 = input.trim().parse().expect("Invalid input.");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let operation: char = input.trim().chars().next().expect("Invalid input.");

    println!("Enter the second number:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let second_number: f64 = input.trim().parse().expect("Invalid input.");

    let operation_enum = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operation."),
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}
