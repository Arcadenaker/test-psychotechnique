use pause_console::*;
use std::io;
use std::io::Write;

mod calculation_test;

fn main() {
    println!("\n----------- Test psychotechnique de l'armée belge -----------\n");
    println!("1) Commencer le test d'aisance numérique");

    let mut number_to_start: String = String::new();
    print!("> ");
    io::stdout().flush().expect("Unable to flush");

    io::stdin()
        .read_line(&mut number_to_start)
        .expect("Unable to read the input");
    let number_to_start: u8 = number_to_start
        .trim()
        .parse()
        .expect("Vous devez rentrer un nombre");

    match number_to_start {
        1 => {
            clearscreen::clear().unwrap();
            println!("-------------- Bienvenue dans le test d'aisance numérique --------------\n");
            println!("Une première opération est présentée. Calculatez mentalement la solution et rentrez-la.");
            println!("Un deuxième calcul est affiché sur un AUTRE écran.");
            println!("Et dans le troisième et dernier écran, indiquez laquelle des deux réponses est la plus ÉLEVÉE.");
            println!("Le premier calcul (Supérieur), deuxième calcul (Inférieur), les deux réponses sont égales (Églale)");
            println!("DURÉE: 90s\n");
            pause_console!("Appuyez sur entrer pour commencer...");

            calculation_test::calculation_test();
        }
        _ => return,
    };
}
