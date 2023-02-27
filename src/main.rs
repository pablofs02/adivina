use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("¡Adivina el número secreto!");

    let número_secreto = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Inserte su número:");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Falló al leer el número.");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match num.cmp(&número_secreto) {
            Ordering::Less => println!("¡Demasiado pequeño!"),
            Ordering::Greater => println!("¡Demasiado grande!"),
            Ordering::Equal => {
                println!("¡Tú ganas!");
                break;
            }
        }
    }
}
