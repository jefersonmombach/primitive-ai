pub fn run() {
    println!("Running Perceptron");

    let tests = [([0, 0], 0), ([0, 1], 0), ([1, 0], 0), ([1, 1], 1)];

    let mut weights = [0, 0];
    let mut bias = 0;

    println!("\nInitial weights: {:?}", weights);
    println!("Initial bias: {}", bias);

    train(&tests, &mut weights, &mut bias);

    println!("\nFinal weights: {:?}", weights);
    println!("Final bias: {}", bias);

    println!("\nTesting:");

    for (inputs, expected) in tests {
        let output = predict(&inputs, &weights, bias);

        println!("{:?} -> {} (expected: {})", inputs, output, expected);
    }
}

fn predict(inputs: &[i32], weights: &[i32], bias: i32) -> i32 {
    let sum: i32 = inputs
        .iter()
        .zip(weights.iter())
        .map(|(input, weight)| input * weight)
        .sum();

    if sum + bias >= 0 { 1 } else { 0 }
}

fn train(tests: &[([i32; 2], i32)], weights: &mut [i32; 2], bias: &mut i32) {
    let learning_rate = 1;
    let max_epochs = 100;

    for epoch in 0..max_epochs {
        let mut errors = 0;

        println!("\nEpoch {}", epoch + 1);

        for (inputs, expected) in tests {
            let predicted = predict(inputs, weights, *bias);

            let error = expected - predicted;

            if error != 0 {
                errors += 1;
            }

            weights[0] += learning_rate * error * inputs[0];
            weights[1] += learning_rate * error * inputs[1];

            *bias += learning_rate * error;

            println!(
                "input: {:?}, expected: {}, produced: {}, error: {}, weights: {:?}, bias: {}",
                inputs, expected, predicted, error, weights, bias
            );
        }

        if errors == 0 {
            println!("\nTraining finished!");
            break;
        }
    }
}
