use rand::{distributions::Uniform, Rng};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn genera_password(lunghezza: usize) -> String {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    rand::thread_rng()
        .sample_iter(&Uniform::new_inclusive(0, chars.len() - 1))
        .take(lunghezza)
        .map(|i| chars.chars().nth(i).unwrap())
        .collect()
}

fn salva_password(file: &str, password: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true) 
        .open(file)?;

    writeln!(file, "{}", password)
}

fn leggi_password(file: &str) -> std::io::Result<String> {
    fs::read_to_string(file)
}

fn main() {
    let file = "passwords.txt";

    loop {
        println!("Generatore di Password Sicure");
        println!("1. Genera una nuova password");
        println!("2. Leggi password salvate");
        println!("3. Esci");
        println!("Scegli un'opzione:");

        let mut scelta = String::new();
        io::stdin().read_line(&mut scelta).unwrap();

        match scelta.trim() {
            "1" => {
                println!("Inserisci la lunghezza della password:");
                let mut lunghezza = String::new();
                io::stdin().read_line(&mut lunghezza).unwrap();

                if let Ok(lunghezza) = lunghezza.trim().parse::<usize>() {
                    let password = genera_password(lunghezza);
                    println!("Password generata: {}", password);

                    if let Err(e) = salva_password(file, &password) {
                        eprintln!("Errore nel salvataggio: {}", e);
                    } else {
                        println!("Password salvata in {}", file);
                    }
                } else {
                    println!("Lunghezza non valida.");
                }
            }
            "2" => match leggi_password(file) {
                Ok(contenuto) => println!("Password salvate:\n{}", contenuto),
                Err(e) => eprintln!("Errore nella lettura: {}", e),
            },
            "3" => break,
            _ => println!("Opzione non valida."),
        }
    }
}