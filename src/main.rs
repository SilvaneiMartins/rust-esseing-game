use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Jogo de Adivinhação!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("O número secreto é: {}", secret_number);

    loop {
        println!("Por favor, digite seu palpite.");
        let mut palpite = String::new();
    
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a linha");
        
        if palpite.trim() == "quit" {
            println!("Você desistiu!");
            break;
        }
        
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    
        println!("Seu palpite: {}", palpite);
    
        match palpite.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            },
        }
    }
}
