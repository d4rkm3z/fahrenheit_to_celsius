use std::io;

fn convert(number: f64) -> f64 {
    (number - 32.0) / 1.8
}

fn main() {
    loop {
        println!("Enter a fahrenheit temperature value to convert celsius");
        println!("or type 'exit' for close program");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Not a number");

        if number.trim() == "exit" {
            break;
        }

        let number: f64 = number.trim()
            .parse()
            .unwrap();

        println!("{} fahrenheit => {:.2} celsius", number, convert(number));
    }
}
