mod search_algs;

use std::io;
use search_algs::search as search;


fn main() {
    // Print the welcome message
    println!("Pick a search algorithm. Enter the number of the algorithm you want to use.");

    // Print the list of algorithms
    println!("1. Linear Search");
    println!("2. Binary Search");

    // Initialise choice to an empty string
    let mut choice = String::new();

    // Read the user's choice
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Check if the user's choice is valid
    // If it is not valid, ask the user to enter a valid choice
    loop {
        if choice.trim() == "1" || choice.trim() == "2" {
            break;
        } else {
            println!("Invalid choice. Please enter 1 or 2.");
            choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
        }
    }
        
    // Convert the user's choice to a u32
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    // Ask the user to enter the size of the array
    println!("Now choose the size of the array.");

    // Initialise size to an empty string
    let mut size = String::new();

    // Read the user's choice
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    // Convert the user's choice to a u32
    let size: u32 = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    // Initialise the result variable
    let mut result = search::SearchResult {
        time: 0.0,
        loop_count: 0,
    };

    // Initialise loop counter i to 0
    let mut i = 0;

    // Choose the algorithm based on the user's choice
    if choice == 1 {
        // Run the algorithm 10000 times
        while i < 10000 {
            // Run the algorithm and store the result in temp
            let temp = search::linear_search(size);

            // Add the time taken and the number of loops to the result variable
            result.time += temp.time;
            result.loop_count += temp.loop_count;

            // Increment i
            i += 1;
        }
        // Divide the number of loops by 10000 to get the average
        result.loop_count /= 10000;

        // Print the average time taken and the average number of loops
        println!("Total time taken: {} seconds", result.time);
        println!("Average number of loops: {}", result.loop_count);
        println!("Note: The average time and average number of loops are calculated over 10000 runs.");
    } else if choice == 2 {
        // Run the algorithm 10000 times
        while i < 10000 {
            // Run the algorithm and store the result in temp
            let temp = search::binary_search(size);

            // Add the time taken and the number of loops to the result variable
            result.time += temp.time;
            result.loop_count += temp.loop_count;

            // Increment i
            i += 1;
        }
        // Divide the number of loops by 10000 to get the average
        result.loop_count /= 10000;

        // Print the average time taken and the average number of loops
        println!("Total time taken: {} seconds", result.time);
        println!("Average number of loops: {}", result.loop_count);
        println!("Note: The average time and average number of loops are calculated over 10000 runs.");
    }
}