use std::io::Write;
use std::{io, thread::sleep, time::Duration};
mod calculation_test;

fn main() {
    begin();
}

fn begin() {
    println!("\n----------- Welcome to the psychotechnic test -----------\n");
    println!("1) Start the mental calculation test");

    let mut number_to_start: String = String::new();
    print!("> ");
    io::stdout().flush().expect("Unable to flush");

    io::stdin()
        .read_line(&mut number_to_start)
        .expect("Unable to read the input");
    let number_to_start: u8 = number_to_start
        .trim()
        .parse()
        .expect("You have to enter a number");

    match number_to_start {
        1 => {
            std::process::Command::new("clear").status().unwrap();
            println!("\n-------------- Welcome in the calculation test --------------\n");
            println!("You have to calculate the 2 calculations that will appear on the screen");
            println!("At the end you should state which is HIGHER (S: first | I: second) or if they are equal (E)");
            println!("The DURATION of the test is 90s");
            sleep(Duration::from_secs(6));

            calculation_test::calculation_test();
        }
        _ => begin(),
    };
}
