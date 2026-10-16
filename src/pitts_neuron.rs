pub fn run() {
    println!("Running artificial neuron of McCulloch-Pitts");

    fn run_tests(tests: &[([i32; 2], i32)], weights: &[i32; 2], threshold: i32) {
        for &(inputs, expected) in tests {
            let output = pitts_neuron(&inputs, weights, threshold);

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

fn pitts_neuron(inputs: &[i32], weights: &[i32], threshold: i32) -> i32 {
    let mut sum = 0;

    for i in 0..inputs.len() {
        sum += inputs[i] * weights[i];
    }

    if sum >= threshold { 1 } else { 0 }
}
