// Step 1: Create an enum called Operation with variants Add, Subtract, Multiply, and Divide.
// Each variant should hold two f64 values.
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Step 2: Create a function called calculate that takes an Operation enum as an argument
// and returns an f64 result.
fn calculate(op: Operation) -> f64 {
    // Step 3: Implement the calculate function using pattern matching to perform
    // the appropriate arithmetic operation based on the variant of the Operation enum.
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                println!("Error: Division by zero.");
                f64::NAN
            }
        }
    }
}

fn main() {
    // Step 4: Prompt the user to input the first number, the operation to be performed,
    // and the second number.
    println!("Enter the first number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    // Step 5: Parse the input values into appropriate variables (first_number, operation, second_number).
    let first_number: f64 = input.trim().parse().expect("Invalid input.");

    println!("Enter the operation (+, -, *, /): ");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).expect("Failed to read input.");
    let operation = operation.trim();

    println!("Enter the second number: ");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let second_number: f64 = input.trim().parse().expect("Invalid input.");

    // Step 6: Create an Operation enum instance with the parsed input values.
    let op = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation.");
            return;
        }
    };

    // Step 7: Call the calculate function with the created Operation enum instance.
    let result = calculate(op);

    // Step 8: Print the result to the console.
    println!("Result: {}", result);
}
