use std::io;

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).expect("❌ Failed to read input.");
    input.trim().to_string()
}

fn parse_number(input: &str) -> Option<f64> {
    input.trim().parse::<f64>().ok()
}

fn calculate(a: f64, b: f64, op: &str) -> Option<f64> {
    match op {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => {
            if b == 0.0 {
                println!("⚠️ Cannot divide by zero!");
                None
            } else {
                Some(a / b)
            }
        }
        "%" => {
            if b == 0.0 {
                println!("⚠️ Cannot perform modulus by zero!");
                None
            } else {
                Some(a % b)
            }
        }
        "^" => Some(a.powf(b)),
        _ => {
            println!("❓ Unknown operator!");
            None
        }
    }
}

fn main() {
    println!("🧮 Welcome to RustCalc 3000!");

    loop {
        let first = get_input("\n👉 Enter the first number (or type 'exit' to quit):");
        if first.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye, happy calculating!");
            break;
        }

        let second = get_input("👉 Enter the second number:");
        let operator = get_input("🧠 Enter an operator (+, -, *, /, %, ^):");

        let a = match parse_number(&first) {
            Some(n) => n,
            None => {
                println!("🚫 That's not a valid number!");
                continue;
            }
        };

        let b = match parse_number(&second) {
            Some(n) => n,
            None => {
                println!("🚫 That's not a valid number!");
                continue;
            }
        };

        if let Some(result) = calculate(a, b, &operator) {
            println!("✅ Result: {} {} {} = {}", a, operator, b, result);
        }

        let again = get_input("\n🔄 Do you want to calculate again? (y/n):");
        if again.eq_ignore_ascii_case("n") || again.eq_ignore_ascii_case("no") {
            println!("👋 Goodbye, see you next time!");
            break;
        }
    }
}