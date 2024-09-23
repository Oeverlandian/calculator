use text_io::read;

fn main() {
    
    loop {

        println!("What do you want to do?");
        println!("1. Single digit calculations");
        println!("2. Double digit calculations");
        println!("3. Triple digit caclulations");
        println!("4. Exit");
        let choice: i8 = read!(); // Choose between options

        match choice {
            1 => one_number_calculations(),
            2 => one_operator_calculation(),
            3 => two_operator_calculation(),
            4 => break,
            _ => println!("Invalid input!"),
        }

    }

}

fn one_number_calculations() {
    print!("Enter the number: ");
    let number: f64 = read!();

    print!("Enter the operator (s for square root, a for sine, c for cosine, t for tangent): ");
    let operator: char = read!();

    match operator {
        's' => println!("{}√={}", number, number.to_radians().sqrt()),
        'a' => println!("sin({})={}", number, number.to_radians().sin()),
        'c' => println!("cos({})={}", number, number.to_radians().cos()),
        't' => println!("tan({})={}", number, number.to_radians().tan()),
        _ =>   println!("Invalid operator.") 
    }
}

fn one_operator_calculation() { // Conducts one operator calculations with a match statement
    print!("Enter the first number: ");
    let number_1: f64 = read!();

    print!("Enter the operator (+, -, *, /, ^): ");
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
        '^' => println!("{}^{}={}", number_1, number_2, number_1.powf(number_2)),
        _ =>   println!("Error: Invalid operator ({}), supported operators are: +, -, *, / and ^", operator),
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
            else { println!("Error: Invalid second operator ({})", operator_2) }
        }
        '-' => {
            if operator_2 == '+' { println!("{}-{}+{}={}", number_1, number_2, number_3, number_1 - number_2 + number_3) }
            else if operator_2 == '-' { println!("{}-{}-{}={}", number_1, number_2, number_3, number_1 - number_2 - number_3)}
            else if operator_2 == '*' { println!("{}-{}*{}={}", number_1, number_2, number_3, number_1 - number_2 * number_3)}
            else if operator_2 == '/' { println!("{}-{}/{}={}", number_1, number_2, number_3, number_1 - number_2 / number_3)}
            else { println!("Error: Invalid second operator ({})", operator_2) }
        }
        '*' => {
            if operator_2 == '+' { println!("{}*{}+{}={}", number_1, number_2, number_3, number_1 * number_2 + number_3) }
            else if operator_2 == '-' { println!("{}*{}-{}={}", number_1, number_2, number_3, number_1 * number_2 - number_3)}
            else if operator_2 == '*' { println!("{}*{}*{}={}", number_1, number_2, number_3, number_1 * number_2 * number_3)}
            else if operator_2 == '/' { println!("{}+{}/{}={}", number_1, number_2, number_3, number_1 * number_2 / number_3)}
            else { println!("Error: Invalid second operator ({})", operator_2) }
        }
        '/' => {
            if operator_2 == '+' { println!("{}/{}+{}={}", number_1, number_2, number_3, number_1 / number_2 + number_3) }
            else if operator_2 == '-' { println!("{}/{}-{}={}", number_1, number_2, number_3, number_1 / number_2 - number_3)}
            else if operator_2 == '*' { println!("{}/{}*{}={}", number_1, number_2, number_3, number_1 / number_2 * number_3)}
            else if operator_2 == '/' { println!("{}/{}/{}={}", number_1, number_2, number_3, number_1 / number_2 / number_3)}
            else { println!("Error: Invalid second operator ({})", operator_2) }
        }
        _ => println!("Error: Invalid first operator ({}), supported operators are: +, -, *, /", operator_1)
    }
    
}