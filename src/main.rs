use pause_console::*;
use dialoguer::{theme::ColorfulTheme, Select};

mod calculation_test;

fn main() {
    println!("\n----------- Test psychotechnique de l'armée belge -----------\n");

    let tests = &[
        "Commencer le test d'aisance numérique",
        "Commencer le test de mémoire des mots",
        "Quitter"
    ];

    let test = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&tests[..])
        .interact()
        .unwrap();

    match test {
        0 => {
            clearscreen::clear().unwrap();
            println!("-------------- Bienvenue dans le test d'aisance numérique --------------\n");
            println!("Une première opération est présentée. Calculatez mentalement la solution et rentrez-la.");
            println!("Un deuxième calcul est affiché sur un AUTRE écran.");
            println!("Et dans le troisième et dernier écran, indiquez laquelle des deux réponses est la plus ÉLEVÉE.");
            println!("Le premier calcul (Supérieur), deuxième calcul (Inférieur), les deux réponses sont égales (Églale)");
            println!("DURÉE DU TEST: 90s\n");
            pause_console!("Appuyez sur entrer pour commencer...");

            calculation_test::calculation_test();
        },
        1 => {
            clearscreen::clear().unwrap();
            println!("-------------- Bienvenue dans le test de mémoire des mots --------------\n");
            println!("Sur le premier écran, 3 règles sont proposées et vous devez les retenir.");
            println!("Sur le deuxième écran, 3 autres mots vous seront présentés. Vous comparez (mentalement) la SUCCESSION des mots avec la SUCCESSION des règles et vous devez décidez combien de mots respèctent les règles.");
            println!("\nEX:");
            println!("Règles: PAYS | VETEMENT | PAYS");
            println!("Mots: Belgique | Chaussette | Angleterre");
            println!("Le premier mot (Belgique) respècte pays, le deuxième (chaussette) est un vêtement et le troisième (Angleterre) est également un pays.");
            println!("La réponse est donc 3.\n");

            println!("\nEX2:");
            println!("Règles: OUTIL | VETEMENT | PAYS");
            println!("Mots: Pantalon | Tractopelle | Bangladesh");
            println!("Le premier mot (Pantalon) n'est pas un outil, le deuxième (Tractopelle) n'est pas un vêtement et le troisième (Bangladesh) est un pays.");
            println!("La réponse est donc 1.\n");
            println!("DURÉE DU TEST: 90s\n");
            pause_console!("Appuyez sur entrer pour commencer...");
        },
        _ => {
            println!("\nEn espérant vous revoir bientôt...");
            return
        },
    };
}
