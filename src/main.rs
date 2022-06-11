use pause_console::*;
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
            //sleep(Duration::from_secs(6));

            calculation_test()
        }
        _ => begin(),
    };
}

fn calculation_test() {
    //Number to make the exercice harder (answers closer)
    let selected_number: u32 = rand::thread_rng().gen_range(10..90);

    fn new_calc(n: u32) -> u32 {
        let sign: u32 = rand::thread_rng().gen_range(1..5);
        match sign {
            1 => add_calc(n),
            2 => sub_calc(n),
            3 => mult_calc(n),
            4 => div_calc(n),
            _ => 0,
        }
    }

    fn add_calc(salt: u32) -> u32 {
        let answer = salt + rand::thread_rng().gen_range(0..5);
        let second_number: u32 = answer - rand::thread_rng().gen_range(4..10);
        let first_number: u32 = answer - second_number;

        println!("{} + {}", first_number, second_number);

        return answer;
    }

    fn sub_calc(salt: u32) -> u32 {
        let answer = salt + rand::thread_rng().gen_range(0..5);
        let first_number: u32 = answer + rand::thread_rng().gen_range(5..10);
        let second_number: u32 = first_number - answer;

        println!("{} - {}", first_number, second_number);

        return answer;
    }

    fn mult_calc(salt: u32) -> u32 {
        let mut facteur_1 = rand::thread_rng().gen_range(3..9);
        let mut answer = find_division_modulo_null(facteur_1, salt);
        let facteur_2 = answer / facteur_1;

        let backup_facteur_1 = facteur_1;
        while facteur_2 > 10 {
            facteur_1 += 1;
            answer = find_division_modulo_null(facteur_1, salt);
            //Pour éviter un bug de leak
            if facteur_1 > 9 {
                facteur_1 = backup_facteur_1;
                continue;
            }
        }

        println!("{} X {}", facteur_1, facteur_2);

        return answer;
    }

    fn find_division_modulo_null(f1: u32, range: u32) -> u32 {
        let possible_range_modulo_null = range + f1;

        for number in range..possible_range_modulo_null {
            if number % f1 == 0 {
                return number;
            }
        }

        return 0;
    }

    fn div_calc(salt: u32) -> u32 {
        let answer = salt + rand::thread_rng().gen_range(0..5);
        let diviseur = rand::thread_rng().gen_range(3..10);
        let dividende = answer * diviseur;

        if dividende > 100 {
            mult_calc(salt);
        }

        println!("{} / {}", dividende, diviseur);

        return answer;
    }

    std::process::Command::new("clear").status().unwrap();
    println!("\n\n\n");

    let first_operation = new_calc(selected_number);
    pause_console!("Press enter to continue...");

    std::process::Command::new("clear").status().unwrap();
    println!("\n\n\n");

    let second_operation = new_calc(selected_number);
    pause_console!("Press enter to continue...");

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
