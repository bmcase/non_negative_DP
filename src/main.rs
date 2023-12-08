use rand::Rng;
use rand::distributions::{Bernoulli, Distribution};
use std::f64::consts;
use std::f64;


fn generate_geometric(probability: f64) -> usize {
    // Create a Bernoulli distribution with the specified success probability
    let bernoulli = Bernoulli::new(probability).expect("Invalid probability");

    // Generate Bernoulli random numbers until the first success
    let mut rng = rand::thread_rng();
    let mut attempts = 0;

    while !bernoulli.sample(&mut rng) {
        attempts += 1;
    }

    attempts
}

fn generate_double_geometric(s: f64, n: usize) -> usize {
    // Call the function and get the result for the first geometric
    let success_probability = 1.0 - consts::E.powf(-1.0 / s);
    let attempts1 = generate_geometric(success_probability);
    // println!("Number of attempts until first success (Probability 1): {}", attempts1);

    // Call the function and get the result for the second geometric
    let attempts2 = generate_geometric(success_probability);
    // println!("Number of attempts until first success (Probability 2): {}", attempts2);

    n + attempts1 - attempts2

}

fn generate_truncated_double_geometric(s: f64, n: usize) -> usize {
    let mut reject = 1;
    let mut sample = 0; // Declare sample here
    while reject == 1 {
        sample = generate_double_geometric(s, n); // Assign a value to sample inside the loop
        if sample > 0 && sample < 2 * n {
            reject = 0
        }
    }
    sample // Return the final value of sample
}

fn main() {
    // Parameters for the function calls
    let epsilon = 1.0;
    let s = 1.0/epsilon;
    let n = 25; //this will determine delta
    // Create a vector to store the samples
    let mut samples = Vec::new();
    // Sample 100 values from the generate_truncated_double_geometric distribution
    for _ in 0..100 {
        let sample = generate_truncated_double_geometric(s, n);
        samples.push(sample);
    }
    // Print the samples to the console
    println!("Samples: {:?}", samples);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_generate_geometric() {
        let probability = 0.5;
        let mut samples = Vec::new();
        // Sample 100 values from the generate_geometric function
        for _ in 0..100 {
            let sample = generate_geometric(probability);
            samples.push(sample);
        }
        // Print the samples to the console
        println!("Samples from generate_geometric: {:?}", samples);
    }
    // #[test]
    // fn test_mean_generate_geometric() {
    //     let probability = 0.5;
    //     let mut samples = Vec::new();
    //     // Sample 100 values from the generate_geometric function
    //     for _ in 0..100 {
    //         let sample = generate_geometric(probability);
    //         samples.push(sample);
    //     }
    //     // Compute the sample mean
    //     let sample_mean = samples.iter().sum::<usize>() as f64 / samples.len() as f64;
    //     // Check that the sample mean is within some distance of the expected value
    //     let expected_mean = 1.0 / probability;
    //     let tolerance = 0.01; // Set the tolerance to 1%
    //     assert!(sample_mean >= expected_mean - tolerance && sample_mean <= expected_mean + tolerance);
    // }
    #[test]
    fn test_generate_geometric() {
        let probability = 0.5;
        let bound = f64::floor(f64::ln(0.001) / f64::ln(1.0 - probability)) as usize;
        println!("Bound: {:?}", bound);
        let mut above_mean = 0;
        let expected_mean = (1.0 - probability)/probability;
        println!("expected_mean: {:?}", expected_mean);
        // Sample 100 values from the generate_geometric function
        for _ in 0..100 {
            let sample = generate_geometric(probability);
            assert!(sample >= 0);
            assert!(sample < bound, "one of 100 samples (sample = {}) exceeded a bound
            (bound = {}) that holds with probability 99.9%.  This test should fail randomly 1% of the time",sample,bound);
            if sample > (f64::ceil(expected_mean) as usize) {
                above_mean += 1;
            }
        }
        assert!(above_mean > 5, "above_mean was {}", above_mean);
    }
    #[test]
    fn test_generate_double_geometric() {
        let s = 1.0;
        let n = 25;
        let mut samples = Vec::new();
        // Sample 100 values from the generate_truncated_double_geometric function
        for _ in 0..100 {
            let sample = generate_double_geometric(s, n);
            samples.push(sample);
        }
        // Print the samples to the console
        println!("Samples from generate_double_geometric with s={}, n={}: {:?}", s, n, samples);
    }
    #[test]
    fn test_generate_truncated_double_geometric() {
        let s = 1.0;
        let n = 25;
        let mut samples = Vec::new();
        // Sample 100 values from the generate_truncated_double_geometric function
        for _ in 0..100 {
            let sample = generate_truncated_double_geometric(s, n);
            assert!(sample > 0 && sample < 2 * n);
            samples.push(sample);
        }
        // Print the samples to the console
        println!("Samples from generate_truncated_geometric with s={}, n={}: {:?}", s, n, samples);
    }
}
