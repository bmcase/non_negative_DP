use rand::Rng;
use rand::distributions::{Bernoulli, Distribution};
use std::f64::consts;
use std::f64;
use std::collections::HashMap;


///************************   Geometric  ************************************************* ///
///*************************************************************************************** ///
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

fn generate_geometric_rng<R: Rng + ?Sized>(probability: f64, rng : &mut R) -> isize {
    // Create a Bernoulli distribution with the specified success probability
    let bernoulli = Bernoulli::new(probability).expect("Invalid probability");

    // Generate Bernoulli random numbers until the first success
    // let mut rng = rand::thread_rng();
    let mut attempts = 0;

    while !bernoulli.sample( rng) {
        attempts += 1;
    }

    attempts
}

#[derive(Debug)]
pub struct Geometric {
    success_probability: f64,
}
impl Geometric {
    /// Creates a new `Geometric` distribution with the given success probability
    pub fn new(success_probability: f64) -> Self {
        Self {
            success_probability,
        }
    }
    /// Generates a sample from the `Geometric` distribution.
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> isize {
        generate_geometric_rng(self.success_probability, rng)
    }
}
impl Distribution<isize> for Geometric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> isize {
        self.sample(rng)
    }
}

///************************  Double Geometric  ******************************************* ///
///*************************************************************************************** ///
fn generate_double_geometric(s: f64, n: isize) -> isize {
    let success_probability = 1.0 - consts::E.powf(-1.0 / s);
    let attempts1 = generate_geometric(success_probability);
    let attempts2 = generate_geometric(success_probability);
    (n + attempts1 - attempts2).try_into().unwrap()
}
fn generate_double_geometric_rng<R: Rng + ?Sized>(s: f64, n: isize, rng: &mut R) -> isize {
    let success_probability = 1.0 - consts::E.powf(-1.0 / s);
    let attempts1 = generate_geometric_rng(success_probability,rng);
    let attempts2 = generate_geometric_rng(success_probability,rng);
    (n + attempts1 - attempts2).try_into().unwrap()
}

#[derive(Debug)]
pub struct DoubleGeometric {
    success_probability: f64,
}
impl DoubleGeometric {
    /// Creates a new `DoubleGeometric` distribution with the given success probability
    pub fn new(success_probability: f64) -> Self {
        Self {
            success_probability,
        }
    }
    /// Generates a sample from the `DoubleGeometric` distribution.
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> isize {
        generate_geometric_rng(self.success_probability, rng)
    }
}
impl Distribution<isize> for DoubleGeometric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> isize {
        self.sample(rng)
    }
}


fn generate_truncated_double_geometric(s: f64, n: isize) -> isize {
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
    let mut rng = rand::thread_rng();
    let p = 0.5;
    generate_geometric_rng(p,&mut rng);


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
    fn test_generate_geometric_sample_dist() {
        let p = 0.5; // success probability
        let mut histogram = HashMap::new();
        let mut num_samples = 100000;

        for _ in 0..num_samples {
            let sample = generate_geometric(p);
            *histogram.entry(sample).or_insert(0) += 1;
        }


        for x in 0..100 {
            let observed_probability = histogram.get(&x).map_or(0.0, |count| *count as f64 / num_samples as f64);

            let expected_probability = (1.0 - p).powf(x as f64) * p as f64;
            // println!("x = {}, Observed Probability = {}, Expected Probability = {}", x, observed_probability, expected_probability);
            assert!((observed_probability - expected_probability) <= 0.01);
        }
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
        let sample_mean = samples.iter().sum::<isize>() as f64 / samples.len() as f64;
        // println!("sample_mean: {:?}", sample_mean);
        // Check that the sample mean is within some distance of the expected value
        let expected_mean = n as f64;
        // println!("expected_mean: {:?}", expected_mean);
        if sample_mean >= expected_mean - (t as f64) && sample_mean <= expected_mean + (t as f64) {
            true // Return true if the test passes
        } else {
            false // Return false if the test fails
        }
    }
    #[test]
    fn test_generate_truncated_double_geometric_sample_dist() {
        let epsilon = 1.0;
        let s = 1.0 / epsilon;
        let n = 25 as isize;
        let num_samples = 100000;
        let mut samples = Vec::new();
        // Sample 1000 values from the generate_truncated_double_geometric function
        for _ in 0..num_samples {
            let sample = generate_truncated_double_geometric(s, n) as isize;
            assert!(sample >= 0 && sample <=(2 * n).try_into().unwrap());
            samples.push(sample);
        }
        // Compute the observed probability for each value in the range [0, 2*n)
        let mut histogram = HashMap::new();
        for value in samples {
            *histogram.entry(value).or_insert(0) += 1;
        }
        let mut sorted_keys: Vec<isize> = histogram.keys().cloned().collect();
        sorted_keys.sort();
        // Compute the expected probability for each value in the range [0, 2*n]
        let normalizing_factor = (1.0 - consts::E.powf(-epsilon)) /
        (1.0 + consts::E.powf(-epsilon) - 2.0 * consts::E.powf(-epsilon * ((n + 1) as f64)));  // 'A' in paper
        // println!("A = {}", normalizing_factor);
        // Compare the observed and expected probabilities for each value in the range [0, 2*n]
        for x  in 0..2*n+1 {
            let observed_probability = histogram.get(&x).map_or(0.0, |count| *count as f64 / num_samples as f64);
            let expected_probability = normalizing_factor * consts::E.powf(-epsilon * ((n - x).abs() as f64));
            // println!("x, prob: {}, {}",x,expected_probability);
            // println!("Value: {}, Observed Probability: {:.4}, Expected Probability: {:.4}", x, observed_probability, expected_probability);
            assert!((observed_probability - expected_probability).abs() <= 0.01, "Observed probability is not within 1% of expected probability");
        }
    }
    #[test]
    fn test_geometric() {
        let mut rng = rand::thread_rng();
        let distribution = Geometric {
            success_probability: 0.5,
        };
        distribution.sample(&mut rng);
    }
    #[test]
    fn test_double_geometric() {
        let mut rng = rand::thread_rng();
        let distribution = DoubleGeometric {
            success_probability: 0.5,
        };
        distribution.sample(&mut rng);
    }

}
