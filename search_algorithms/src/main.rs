mod search_algs;

use std::io;
use search_algs::search as search;


fn main() {
    println!("Pick a search algorithm. Enter the number of the algorithm you want to use.");

    println!("1. Linear Search");
    println!("2. Binary Search");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

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
        
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("Now choose the size of the array you want to search in.");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: u32 = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if choice == 1 {
        let result = search::linear_search(size);
        println!("Time taken: {} seconds", result.time);
        println!("Number of loops: {}", result.loop_count);
    } else if choice == 2 {
        let result = search::binary_search(size);
        println!("Time taken: {} seconds", result.time);
        println!("Number of loops: {}", result.loop_count);
    }
}