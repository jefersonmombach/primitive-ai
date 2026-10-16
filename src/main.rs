mod pitts_neuron;

use std::io;

fn main() {
    loop {
        println!("\n=== MENU ===");
        println!("1. Artificial neuron of McCulloch-Pitts");
        println!("9. Exit");
        print!("Choice?");

        use std::io::Write;
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("input error");

        let choice = choice.trim();

        match choice {
            "1" => run_pitts_neuron(),
            "9" => {
                println!("Exiting");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn run_pitts_neuron() {
    println!("Running artificial neuron of McCulloch-Pitts");

    fn run_tests(tests: &[([i32; 2], i32)], weights: &[i32; 2], threshold: i32) {
        for &(inputs, expected) in tests {
            let output = pitts_neuron::pitts_neuron(&inputs, weights, threshold);

            println!("{:?} -> {} (esperado: {})", inputs, output, expected);
        }
    }

    println!("========== AND ==========");
    let and_weights = [1, 1];
    let and_threshold = 2;
    let and_tests = [([0, 0], 0), ([0, 1], 0), ([1, 0], 0), ([1, 1], 1)];
    run_tests(&and_tests, &and_weights, and_threshold);

    println!("========== OR ===========");
    let or_weights = [1, 1];
    let or_threshold = 1;
    let or_tests = [([0, 0], 0), ([0, 1], 1), ([1, 0], 1), ([1, 1], 1)];
    run_tests(&or_tests, &or_weights, or_threshold);
}
