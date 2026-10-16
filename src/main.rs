mod pitts_neuron;
mod perceptron;

use pitts_neuron::*;
use perceptron::*;
use std::io;

fn main() {
    loop {
        println!("\n=== MENU ===");
        println!("1. Artificial neuron of McCulloch-Pitts");
        println!("2. Perceptron");
        println!("9. Exit");
        print!("Choice?");

        use std::io::Write;
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("input error");

        let choice = choice.trim();

        match choice {
            "1" => pitts_neuron::run(),
            "2" => perceptron::run(),
            "9" => {
                println!("Exiting");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

