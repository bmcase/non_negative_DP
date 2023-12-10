use rand::Rng;
use rand::distributions::{Bernoulli, Distribution};
use std::f64::consts;
use std::f64;
use std::collections::HashMap;



fn generate_geometric(probability: f64) -> isize {
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

fn generate_double_geometric(s: f64, n: isize) -> isize {
    let success_probability = 1.0 - consts::E.powf(-1.0 / s);
    let attempts1 = generate_geometric(success_probability);
    let attempts2 = generate_geometric(success_probability);
    (n + attempts1 - attempts2).try_into().unwrap()

}

fn generate_truncated_double_geometric(s: f64, n: isize) -> usize {
    let mut reject = 1;
    let mut sample = 0; // Declare sample here
    while reject == 1 {
        sample = generate_double_geometric(s, n); // Assign a value to sample inside the loop
        if sample >= 0 && sample <= (2 * n).try_into().unwrap()  {
            reject = 0
        }
    }
    sample.try_into().unwrap()  // Return the final value of sample
}

fn main() {
    // Parameters for the function calls
    let epsilon = 0.01;
    let s = 1.0/epsilon;
    let n = 50; //this will determine delta
    // Create a vector to store the samples
    let mut samples = Vec::new();
    // Sample 100 values from the generate_truncated_double_geometric distribution
    for _ in 0..100000 {
        let sample = generate_truncated_double_geometric(s, n);
        samples.push(sample);
    }
    // Print the samples to the console
    // println!("Samples: {:?}", samples);
    let mut histogram = HashMap::new();
    for value in samples {
        *histogram.entry(value).or_insert(0) += 1;
    }
    let mut sorted_keys: Vec<usize> = histogram.keys().cloned().collect();
    sorted_keys.sort();
    println!("Histogram:");
    for key in sorted_keys {
        println!("{}, {}", key, histogram[&key]);
    }
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
    fn test_generate_double_geometric_variance() {
        let s = 1.0; // Set s to some value (e.g., 1.0)
        let n = 25; // Set n to some value (e.g., 25)
        let mut samples = Vec::new();
        for _ in 0..1000 {
            let sample = generate_double_geometric(s, n);
            samples.push(sample);
        }
        let sample_mean = samples.iter().sum::<usize>() as f64 / samples.len() as f64;
        let sample_variance = samples.iter().map(|x| ((*x as f64) - &sample_mean).powf(2.0)).sum::<f64>() / (samples.len() as f64 - 1.0);
        let success_probability = 1.0 - consts::E.powf(-1.0 / s);
        let expected_variance = 2.0 * (1.0-success_probability)/f64::powf(success_probability,2.0);
        println!("Sample variance: {}, Expected variance {}", sample_variance,expected_variance);
        let tolerance = 0.05; // Set the tolerance to 5%
        assert!(sample_variance >= expected_variance * (1.0 - tolerance) && sample_variance <= expected_variance * (1.0 + tolerance),
        "Sample variance ({}) is not within {}% of expected variance ({})", sample_variance, tolerance * 100.0, expected_variance);
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
    #[test]
    fn test_generate_truncated_double_geometric_hoffding() {
        assert!(test_internal_generate_truncated_double_geometric_hoffding());
    }
    fn test_internal_generate_truncated_double_geometric_hoffding() -> bool {
        let number_samples = 1000;
        let failure_prob = 1.0;
        let s = 1.0; // Set s to some value (e.g., 1.0)
        let n = 25; // Set n to some value (e.g., 25)
        let t = f64::sqrt( f64::powf(2.0 * (n as f64),2.0) / (- 2.0 * (number_samples as f64)) * f64::ln(failure_prob/2.0));
        println!("t: {:?}", t);

        let mut samples = Vec::new();
        // Sample number_samples values from the generate_truncated_double_geometric function
        for _ in 0..number_samples {
            let sample = generate_truncated_double_geometric(s, n);
            samples.push(sample);
        }
        // Compute the sample mean
        let sample_mean = samples.iter().sum::<usize>() as f64 / samples.len() as f64;
        println!("sample_mean: {:?}", sample_mean);
        // Check that the sample mean is within some distance of the expected value
        let expected_mean = n as f64;
        println!("expected_mean: {:?}", expected_mean);
        if sample_mean >= expected_mean - (t as f64) && sample_mean <= expected_mean + (t as f64) {
            true // Return true if the test passes
        } else {
            false // Return false if the test fails
        }
    }
    #[test]
    fn test_failure_prob_of_test_truncated_double_geometric() {
        // this test tests how often the test_internal_generate_truncated_double_geometric_hoffding test is
        // actually failing. It may fail significantly fewer times than the bound set since the inequality is not
        // necessarily tight.
        let mut failed = 0;
        let total_tests = 100;
        for _ in 0..total_tests {
            if !test_internal_generate_truncated_double_geometric_hoffding() {
                failed += 1;
            }
        }
        println!("{} of tests failed out of {}", failed, total_tests);
    }
}
