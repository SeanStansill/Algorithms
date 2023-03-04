// Import timing library and random number generator
use std::time::Instant;
use rand::Rng;

// Define a struct to store the results of the search
pub struct SearchResult {
    pub time: f64,
    pub loop_count: u32,
}

// Define the linear search algorithm
pub fn linear_search(n: u32) -> SearchResult {
    // Initialise loop_count to 0
    let mut loop_count = 0;
    
    // Create an integer array of size n
    let array = vec![1; n as usize];

    // Instantiate a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and n
    let target = rng.gen_range(1..=n);

    // Start the timer
    let start = Instant::now();

    // Loop through the array
    while loop_counter < n {

        // Increment loop_count by 1 (must be incremented before testing the condition)
        loop_count += 1;

        // If the current element is equal to the target, break out of the loop
        if array[i as usize] == target {
            break;
        }
    }

    // Stop the timer
    let end = Instant::now();
    
    // Calculate the time taken
    let time = end.duration_since(start).as_secs_f64();
    
    // Return the time taken and the number of loops
    SearchResult {
        time,
        loop_count,
    }
}

// Define the binary search algorithm
pub fn binary_search(n: u32) -> SearchResult {    
    // Initialise loop_count to 0
    let mut loop_count = 0;

    // Create an integer array of size n
    let array = vec![1; n as usize];

    // Instantiate a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and n
    let target = rng.gen_range(1..=n);

    // Start the timer
    let start = Instant::now();

    // Set low to 0 and high to n - 1
    let mut low = 0;
    let mut high = n - 1;

    // Loop while low is less than or equal to high
    while low <= high {

        // Increment loop_count by 1
        loop_count += 1;

        // Calculate the mid point
        // Rust's internal integer division is floored. No need to use floor() or handle floats
        let mid = (low + high) / 2;
        
        // If the element at the mid point is equal to the target, break out of the loop
        // Otherwise, if the element at the mid point is less than the target, set low to mid + 1
        // Otherwise, set high to mid - 1
        if array[mid as usize] == target {
            break;
        } else if array[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    // Stop the timer
    let end = Instant::now();

    // Calculate the time taken
    let time = end.duration_since(start).as_secs_f64();
    
    // Return the time taken and the number of loops
    SearchResult {
        time,
        loop_count,
    }
}