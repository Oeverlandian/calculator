use text_io::read;

fn main() {

    println!("What do you want to do?");
    println!("1. 1 Operator Calculations");
    println!("2. 2 Operator Caclulations");
    let choice: i8 = read!(); // Choose between options

    match choice {
        1 => one_operator_calculation(),
        2 => two_operator_calculation(),
        _ => println!("Invalid Input!"),
    }

}

fn one_operator_calculation() { // Conducts one operator calculations with a match statement
    print!("Enter the first number: ");
    let number_1: f64 = read!();

    print!("Enter the operator (+, -, *, /): ");
    let operator: char = read!();
    
    print!("Enter the second number: ");
    let number_2: f64 = read!();

    match operator {
        '+' => println!("{}+{}={}", number_1, number_2, number_1 + number_2),
        '-' => println!("{}-{}={}", number_1, number_2, number_1 - number_2),
        '*' => println!("{}*{}={}", number_1, number_2, number_1 * number_2),
        '/' => {
            if number_2 == 0.0 {
                println!("Error: Can't divide by zero. {}/{} is not allowed", number_1, number_2);
            } else {
                println!("{}/{}={}", number_1, number_2, number_1 + number_2);
                }   
            },
            _ => println!("Error: Invalid operator!"),
    }
    
}

fn two_operator_calculation() { // Conducts two operator calculations using match statements with if/if else statements in them
    print!("Enter the first number: ");
    let number_1: f64 = read!();

    print!("Enter the first operator (+, -, *, /): ");
    let operator_1: char = read!();
    
    print!("Enter the second number: ");
    let number_2: f64 = read!();

    print!("Enter the first operator (+, -, *, /): ");
    let operator_2: char = read!();

    print!("Enter the third number: ");
    let number_3: f64 = read!();

    match operator_1 {
        '+' => {
            if operator_2 == '+' { println!("{}+{}+{}={}", number_1, number_2, number_3, number_1 + number_2 + number_3) }
            else if operator_2 == '-' { println!("{}+{}-{}={}", number_1, number_2, number_3, number_1 + number_2 - number_3)}
            else if operator_2 == '*' { println!("{}+{}*{}={}", number_1, number_2, number_3, number_1 + number_2 * number_3)}
            else if operator_2 == '/' { println!("{}+{}/{}={}", number_1, number_2, number_3, number_1 + number_2 / number_3)}
            else { println!("Error: Invalid Second Operator ({})", operator_2) }
        }
        '-' => {
            if operator_2 == '+' { println!("{}-{}+{}={}", number_1, number_2, number_3, number_1 - number_2 + number_3) }
            else if operator_2 == '-' { println!("{}-{}-{}={}", number_1, number_2, number_3, number_1 - number_2 - number_3)}
            else if operator_2 == '*' { println!("{}-{}*{}={}", number_1, number_2, number_3, number_1 - number_2 * number_3)}
            else if operator_2 == '/' { println!("{}-{}/{}={}", number_1, number_2, number_3, number_1 - number_2 / number_3)}
            else { println!("Error: Invalid Second Operator ({})", operator_2) }
        }
        '*' => {
            if operator_2 == '+' { println!("{}*{}+{}={}", number_1, number_2, number_3, number_1 * number_2 + number_3) }
            else if operator_2 == '-' { println!("{}*{}-{}={}", number_1, number_2, number_3, number_1 * number_2 - number_3)}
            else if operator_2 == '*' { println!("{}*{}*{}={}", number_1, number_2, number_3, number_1 * number_2 * number_3)}
            else if operator_2 == '/' { println!("{}+{}/{}={}", number_1, number_2, number_3, number_1 * number_2 / number_3)}
            else { println!("Error: Invalid Second Operator ({})", operator_2) }
        }
        '/' => {
            if operator_2 == '+' { println!("{}/{}+{}={}", number_1, number_2, number_3, number_1 / number_2 + number_3) }
            else if operator_2 == '-' { println!("{}/{}-{}={}", number_1, number_2, number_3, number_1 / number_2 - number_3)}
            else if operator_2 == '*' { println!("{}/{}*{}={}", number_1, number_2, number_3, number_1 / number_2 * number_3)}
            else if operator_2 == '/' { println!("{}/{}/{}={}", number_1, number_2, number_3, number_1 / number_2 / number_3)}
            else { println!("Error: Invalid Second Operator ({})", operator_2) }
        }
        _ => println!("Error: Invalid First Operator ({})", operator_1)
    }
    
}
