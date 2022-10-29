use pause_console::*;
use rand::Rng;
use std::{cmp::Ordering,time::Duration, time::Instant};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{Write, stdout};
use crossterm::{execute, QueueableCommand, cursor};   

//Debug is usefull to compare calculations between them (equal, not equal)
#[derive(Debug)]
enum Calculation {
    Addition(u32, u32),
    Substraction(u32, u32),
    Division(u32, u32),
    Multiplication(u32, u32),
}

impl Calculation {
    fn show(&self) {
        match self {
            Calculation::Addition(a, b) => {
                println!("{} + {}", a, b)
            }
            Calculation::Substraction(a, b) => {
                println!("{} - {}", a, b)
            }
            Calculation::Division(a, b) => {
                println!("{} / {}", a, b)
            }
            Calculation::Multiplication(a, b) => {
                println!("{} * {}", a, b)
            }
        }
    }
    fn get_answer(&self) -> u32 {
        match self {
            Calculation::Addition(a, b) => a + b,
            Calculation::Substraction(a, b) => a - b,
            Calculation::Division(a, b) => a / b,
            Calculation::Multiplication(a, b) => a * b,
        }
    }

    fn equal(&self, other: &Calculation) -> bool {
        return format!("{:?}", self) == format!("{:?}", other);
    }
}

pub fn calculation_test() {
    fn new_calc(n: u32) -> Calculation {
        let sign: u32 = rand::thread_rng().gen_range(1..5);
        match sign {
            1 => add_calc(n),
            2 => sub_calc(n),
            3 => mult_calc(n),
            _ => div_calc(n),
        }
    }

    fn add_calc(salt: u32) -> Calculation {
        let answer = salt + rand::thread_rng().gen_range(0..5);
        let second_number: u32 = answer - rand::thread_rng().gen_range(5..15);
        Calculation::Addition(answer - second_number, second_number)
    }

    fn sub_calc(salt: u32) -> Calculation {
        let answer = salt + rand::thread_rng().gen_range(0..5);
        let first_number: u32 = answer + rand::thread_rng().gen_range(5..10);
        Calculation::Substraction(first_number, first_number - answer)
    }

    fn mult_calc(salt: u32) -> Calculation {
        let mut facteur_1 = rand::thread_rng().gen_range(4..9);
        let mut answer = find_division_modulo_null(facteur_1, salt);
        let mut facteur_2 = answer / facteur_1;

        //Calculer 2 facteurs de la multiplication en dessous de 10
        if facteur_2 > 12 {
            facteur_1 = rand::thread_rng().gen_range(1..4);
            while facteur_2 > 12 {
                facteur_1 += 1;
                answer = find_division_modulo_null(facteur_1, salt);
                facteur_2 = answer / facteur_1;

                //Pour éviter un calcul infini si aucun facteurs donnent une réponse proche du
                //"salt"
                if facteur_1 > 12 {
                    add_calc(salt);
                }
            }
        }

        Calculation::Multiplication(facteur_1, facteur_2)
    }

    fn div_calc(salt: u32) -> Calculation {
        let mut diviseur = 2;
        let mut quotient = salt + rand::thread_rng().gen_range(0..3);
        let mut dividende = quotient * diviseur;

        while dividende > 100 {
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
        Calculation::Division(dividende, diviseur)
    }

    //Fonction qui cherche un diviseur modulo 0
    fn find_division_modulo_null(f1: u32, range: u32) -> u32 {
        let possible_range_modulo_null = range + f1;

        for number in range..possible_range_modulo_null {
            if number % f1 == 0 {
                return number;
            }
        }

        0b0
    }

    //Démarre le chronomètre
    let start = Instant::now();

    let mut level = 0; //Numérote les éxos-

    let mut right_answer = 0;
    let mut wrong_answer = 0;

    let mut stdout = stdout();

    loop {
        level += 1;

        //Number to make the exercice harder (answers closer)
        let base: u32 = rand::thread_rng().gen_range(15..89);

        clearscreen::clear().unwrap();

        execute!(stdout,cursor::MoveTo(15, 10));
        println!("      #{}", level);
        execute!(stdout,cursor::MoveTo(15, 11));
        print!("[1]  ");
        let first_operation = new_calc(base);
        let mut second_operation: Calculation;

        //Permet d'éviter les mêmes calculs
        loop {
            second_operation = new_calc(base);
            if !first_operation.equal(&second_operation) {
                break;
            }
        }

        first_operation.show();
        pause_console!("Appuyez sur enter...");
        clearscreen::clear().unwrap();


        execute!(stdout,cursor::MoveTo(15, 11));
        print!("[2]  ");
        second_operation.show();
        pause_console!("Appuyez sur enter...");

        clearscreen::clear().unwrap();

        let selections = &[
            "Supérieur",
            "Inférieur",
            "Églale",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Quelle est la réponse la plus élevée?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        let answer = match first_operation
            .get_answer()
            .cmp(&second_operation.get_answer())
        {
            Ordering::Less => String::from("Inférieur"),
            Ordering::Greater => String::from("Supérieur"),
            Ordering::Equal => String::from("Églale"),
        };

        if selections[selection] == answer {
            right_answer += 1;
        } else {
            wrong_answer += 1;
        }

        if start.elapsed() > Duration::from_secs(90) {
            clearscreen::clear().unwrap();
            println!("--- Vous avez fini le test ---");
            println!(
                "Bonnes réponses: {}",
                right_answer
            );
            println!(
                "Mauvaises réponses: {}",
                wrong_answer
            );
            break;
        }
    }
}