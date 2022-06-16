use pause_console::*;
use std::io;
use std::io::Write;

mod calculation_test;

fn main() {
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
            print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
            println!("-------------- Welcome in the calculation test --------------\n");
            println!("A first operation appears. Calculate it and remember the answer.");
            println!("A second operation is then displayed on ANOTHER screen");
            println!("On the third and final screen, indicate which of the two solutions is the GREATEST");
            println!("DURATION: 90s\n");
            pause_console!("Press enter to start...");

            calculation_test::calculation_test();
        }
        _ => return,
    };
}
