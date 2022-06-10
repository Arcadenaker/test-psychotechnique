use std::{cmp::Ordering, io, io::Write, thread::sleep, time::Duration};

use rand::Rng;

fn main() {
    begin();
}

fn begin() {
    println!("\n-----------Welcome to the psychotechnic test-----------");
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
            println!("\n----Welcome in the calculation test----");
            println!("You have to calculate the 2 calculations that will appear on the screen");
            println!("At the end you should state which is higher (S: first | I: second) or if they are equal (E)");
            sleep(Duration::from_secs(7));

            calculation_test()
        }
        _ => begin(),
    };
}

fn calculation_test() {
    fn new_calc() -> u32 {
        let sign: u32 = rand::thread_rng().gen_range(1..5);
        match sign {
            1 => add_calc(),
            2 => sub_calc(),
            3 => mult_calc(),
            4 => div_calc(),
            _ => 0,
        }
    }

    fn add_calc() -> u32 {
        let first_number: u32 = rand::thread_rng().gen_range(1..101);
        let second_number: u32 = rand::thread_rng().gen_range(1..first_number);

        println!("{} + {}", first_number, second_number);

        return first_number + second_number;
    }

    fn sub_calc() -> u32 {
        let first_number: u32 = rand::thread_rng().gen_range(40..101);
        let second_number: u32 = rand::thread_rng().gen_range(1..first_number);

        println!("{} - {}", first_number, second_number);

        return first_number - second_number;
    }

    fn mult_calc() -> u32 {
        let first_number: u32 = rand::thread_rng().gen_range(1..11);
        let second_number: u32 = rand::thread_rng().gen_range(1..11);

        println!("{} X {}", first_number, second_number);

        return first_number * second_number;
    }

    fn div_calc() -> u32 {
        let diviseur: u32 = rand::thread_rng().gen_range(1..11);
        let quotient: u32 = rand::thread_rng().gen_range(1..11);
        let dividende: u32 = diviseur * quotient;

        println!("{} / {}", dividende, diviseur);

        return quotient;
    }

    std::process::Command::new("clear").status().unwrap();
    println!("\n\n\n");

    let first_operation = new_calc();
    sleep(Duration::from_secs(3));

    std::process::Command::new("clear").status().unwrap();
    println!("\n\n\n");

    let second_operation = new_calc();
    sleep(Duration::from_secs(3));

    std::process::Command::new("clear").status().unwrap();
    println!("\n\n\n");

    let mut answer_usr = String::new();

    print!("Which one is higher? (S, I or E)?  ");
    io::stdout().flush().expect("Unable to flush");
    io::stdin()
        .read_line(&mut answer_usr)
        .expect("Unable to read user input");

    let answer = match first_operation.cmp(&second_operation) {
        Ordering::Less => String::from("I"),
        Ordering::Greater => String::from("S"),
        Ordering::Equal => String::from("E"),
    };

    if answer_usr.trim() == answer {
        println!("Bravo, try the next one");
        sleep(Duration::from_secs(2));
    } else {
        println!("This is wrong, here is the right answer: {}", answer);
        sleep(Duration::from_secs(2));
    }

    calculation_test();
}
