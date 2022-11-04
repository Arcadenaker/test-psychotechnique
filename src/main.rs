use pause_console::*;
use dialoguer::{theme::ColorfulTheme, Select};

mod calculation_test;

fn main() {
    println!("\n----------- Test psychotechnique de l'armée belge -----------\n");

    let tests = &[
        "Commencer le test d'aisance numérique",
        "Quitter"
    ];

    let test = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&tests[..])
        .interact()
        .unwrap();

    match tests[test] {
        "Commencer le test d'aisance numérique" => {
            clearscreen::clear().unwrap();
            println!("-------------- Bienvenue dans le test d'aisance numérique --------------\n");
            println!("Une première opération est présentée. Calculatez mentalement la solution et rentrez-la.");
            println!("Un deuxième calcul est affiché sur un AUTRE écran.");
            println!("Et dans le troisième et dernier écran, indiquez laquelle des deux réponses est la plus ÉLEVÉE.");
            println!("Le premier calcul (Supérieur), deuxième calcul (Inférieur), les deux réponses sont égales (Églale)");
            println!("DURÉE DU TEST: 90s\n");
            pause_console!("Appuyez sur entrer pour commencer...");

            calculation_test::calculation_test();
        }
        _ => return,
    };
}
