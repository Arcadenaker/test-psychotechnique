use pause_console::*;
use rand::Rng;
use std::{cmp::Ordering, io, io::Write, time::Duration, time::Instant};

pub fn calculation_test() {
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
        let second_number: u32 = answer - rand::thread_rng().gen_range(6..13);
        let first_number: u32 = answer - second_number;

        println!("{} + {}", first_number, second_number);

        return answer;
    }

    fn sub_calc(salt: u32) -> u32 {
        let answer = salt + rand::thread_rng().gen_range(0..5);
        let first_number: u32 = answer + rand::thread_rng().gen_range(5..8);
        let second_number: u32 = first_number - answer;

        println!("{} - {}", first_number, second_number);

        return answer;
    }

    fn mult_calc(salt: u32) -> u32 {
        let mut facteur_1 = rand::thread_rng().gen_range(4..10);
        let mut answer = find_division_modulo_null(facteur_1, salt);
        let mut facteur_2 = answer / facteur_1;

        let backup_facteur_1 = facteur_1;
        let backup_answer = answer;
        let backup_facteur_2 = facteur_2;

        //Calculer 2 facteurs de la multiplication en dessous de 10
        if facteur_2 > 9 {
            facteur_1 = rand::thread_rng().gen_range(1..4);
            while facteur_2 > 9 {
                facteur_1 += 1;
                answer = find_division_modulo_null(facteur_1, salt);
                facteur_2 = answer / facteur_1;

                //Pour éviter un calcul infini
                if facteur_1 > 9 {
                    facteur_1 = backup_facteur_1;
                    facteur_2 = backup_facteur_2;
                    answer = backup_answer;
                    break;
                }
            }
        }

        println!("{} X {}", facteur_1, facteur_2);

        return answer;
    }

    fn div_calc(salt: u32) -> u32 {
        let mut diviseur = 2;
        let mut quotient = salt + rand::thread_rng().gen_range(0..3);
        let mut dividende = quotient * diviseur;

        while dividende > 100 || dividende % diviseur != 0 {
            diviseur += 1;
            quotient = find_division_modulo_null(diviseur, salt);
            dividende = quotient * diviseur;
            if dividende < 100 {
                break;
            }
            if diviseur > 6 {
                return mult_calc(salt);
            }
        }
        println!("{} / {}", dividende, diviseur);

        return quotient;
    }

    //Fonction qui cherche un diviseur modulo 0
    fn find_division_modulo_null(f1: u32, range: u32) -> u32 {
        let possible_range_modulo_null = range + f1;

        for number in range..possible_range_modulo_null {
            if number % f1 == 0 {
                return number;
            }
        }

        return 0;
    }

    let start = Instant::now();

    let mut right_answer = 0;
    let mut wrong_answer = 0;

    let mut advancement_of_operation = 0;

    loop {
        advancement_of_operation += 1;

        //Number to make the exercice harder (answers closer)
        let selected_number: u32 = rand::thread_rng().gen_range(15..89);

        std::process::Command::new("clear").status().unwrap();
        println!("\n\n\n");

        println!("\t   #{}", advancement_of_operation);
        print!("[1]      ");
        let first_operation = new_calc(selected_number);
        pause_console!("Press enter...");

        std::process::Command::new("clear").status().unwrap();
        println!("\n\n\n");

        print!("\n[2]      ");
        let second_operation = new_calc(selected_number);
        pause_console!("Press enter...");

        std::process::Command::new("clear").status().unwrap();
        println!("\n\n\n");

        let mut answer_usr = String::new();

        println!("Which one is higher? (S, I or E)?");
        print!("> ");
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
            right_answer += 1;
        } else {
            wrong_answer += 1;
        }

        if start.elapsed() > Duration::from_secs(90) {
            std::process::Command::new("clear").status().unwrap();
            println!("\n\n\n --- Vous avez fini le test --- \n");
            println!("Bonnes réponses: {}", right_answer);
            println!("Mauvaises réponses: {}", wrong_answer);
            break;
        }
    }
}
