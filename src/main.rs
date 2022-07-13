use std::io;

fn calculate_func(x: f64, y: f64, operator: String) -> f64 {
    // let operator_string = operator.as_str();
    let operator_str= operator.as_str().trim();
    let result = match operator_str {
        "Add" => x + y,
        "Subtract" => x - y,
        "Multiply" => x * y,
        "Divide" => x / y,
        _ => x - y,
    };
    let bool = operator == "Add";
    println!("{} equals Add and is {}" , bool, operator);

    return result;
}

fn main() {
    println!("Enter a number:");
    let mut number_one = String::new();
    let mut number_two = String::new();
    let mut operation = String::new();

    io::stdin()
        .read_line(&mut number_one)
        .expect("failed to readline");
    println!("Enter a second number:");

    io::stdin()
        .read_line(&mut number_two)
        .expect("failed to readline");
        println!("Enter your desired operation (Add, Multiply, Subtract, Divide):");

    io::stdin()
        .read_line(&mut operation)
        .expect("failed to readline");

    let number_one = number_one.trim().parse::<f64>().expect("invalid input");
    let number_two = number_two.trim().parse::<f64>().expect("invalid input");

    let result = calculate_func(number_one, number_two, operation);

    print!(
        "You entered {} and {} and the result is {}",
        number_one, number_two, result
    );
}
