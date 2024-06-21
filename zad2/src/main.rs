pub mod memory;
use u64 as IdType; // for clarity and readability
use crate::memory::Page;

pub mod lru;
use crate::lru::lru;
pub mod lfu;
use crate::lfu::lfu;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};

// Function to load data from a file and initialize the pages
fn load_data(filename: &str, size: usize) -> (Vec<Page>, Vec<IdType>, usize) {
    // Read file content into input_table
    let input_table: Vec<IdType> = {
        let file = File::open(filename).expect("Unable to open file");
        let reader = io::BufReader::new(file);
        let mut input_table = Vec::new();
        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            for num in line.split(',') {
                let num: IdType = num.trim().parse().expect("Unable to parse integer");
                input_table.push(num);
            }
        }
        input_table
    };

    // Create pages vector
    let mut pages: Vec<Page> = Vec::new();
    for &id in &input_table {
        if !pages.iter().any(|value| value.get_id() == id) {
            pages.push(Page::new(id));
        }
    }

    (pages, input_table, size)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <filename> <size>");
        return;
    }
    let filename = &args[1];
    let size: usize = args[2].parse().expect("Invalid size value");

    // Load data from the specified file
    let (page_tab, ref_tab, size) = load_data(filename, size);

    // Uncomment to print the pages and reference table for debugging
    // println!("{:?}", page_tab);
    // println!("{:?}", ref_tab);

    // Run the LRU page replacement algorithm and print the result
    println!("lru, page_fault: {:?}", lru(page_tab.clone(), ref_tab.clone(), size));
    
    // Run the LFU page replacement algorithm and print the result
    println!("lfu, page_fault: {:?}", lfu(page_tab.clone(), ref_tab.clone(), size));
}

