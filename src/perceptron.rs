pub fn run() {
    println!("Running Perceptron");

    let tests = [
        ([0, 0], 0),
        ([0, 1], 0),
        ([1, 0], 0),
        ([1, 1], 1),
    ];

    let mut weights = [0, 0];
    println!("\nInitial weights: {:?}", weights);

    let threshold = 1;

    train(&tests, &mut weights, threshold);

    println!("Final weights: {:?}", weights);
    println!("\nTesting:");
    for (inputs, expected) in tests {
        let output = predict(&inputs, &weights, threshold);

        println!(
            "{:?} -> {} (expected: {})",
            inputs, output, expected
        );
    }
}

fn predict(inputs: &[i32], weights: &[i32], threshold: i32) -> i32 {
    let sum: i32 = inputs
        .iter()
        .zip(weights.iter())
        .map(|(input, weight)| input * weight)
        .sum();

    if sum >= threshold {
        1
    } else {
        0
    }
}

fn train(
    tests: &[([i32; 2], i32)],
    weights: &mut [i32; 2],
    threshold: i32,
) {
    let learning_rate = 1;

    for (inputs, expected) in tests {
        let predicted = predict(inputs, weights, threshold);

        let error = expected - predicted;

        weights[0] += learning_rate * error * inputs[0];
        weights[1] += learning_rate * error * inputs[1];

        println!(
            "input: {:?}, expected: {}, produced: {}, erro:r {}, weights: {:?}",
            inputs, expected, predicted, error, weights
        );
    }
}