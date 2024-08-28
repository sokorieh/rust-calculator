mod calculator;

fn main() {
    let expr = "3 + 5 * (10 - 4) / 2";
    match calculator::Calculator::calculate(expr) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}
