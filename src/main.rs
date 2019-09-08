extern crate rand;
use rand::Rng;
use std::io;
use std::str::FromStr;
use std::io::Write;

fn choisir_nb() -> Option<i32> {
    let mut input = String::new();
    print!("Entrez un nombre : ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return Some(i32::from_str(input.trim()).unwrap());
        },
        _ => {
            println!("Entre un nombre valide !");
            None
        }
    }
}

fn jeu() -> bool {
    println!("Generation du nombre...");
    let nombre_aleatoire = rand::thread_rng().gen_range(1, 101);
    let mut tentative = 10;

    println!("C'est parti !");

    while tentative > 0 {
        tentative -= 1;
        match choisir_nb() {
            Some(nb) => {
                if nb > nombre_aleatoire {
                    println!("-> C'est plus petit");
                } else if nb < nombre_aleatoire {
                    println!("-> C'est plus grand");
                } else {
                    println!("Bravo vous avez gagné !");
                    return true;
                }
            },
            None => {}
        }
    }
    println!("Vous avez perdu !");
    return false;
}

fn main() {
    jeu();
}
