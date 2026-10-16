pub fn pitts_neuron(inputs: &[i32], weights: &[i32], threshold: i32) -> i32 {
    let mut sum = 0;

    for i in 0..inputs.len() {
        sum += inputs[i] * weights[i];
    }

    if sum >= threshold { 1 } else { 0 }
}
