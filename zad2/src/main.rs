pub mod memory;
use u64 as IdType;
use crate::memory::Page;
// use crate::memory::MemSim;
// use std::cell::RefCell;

// pub mod fifo;
// use crate::fifo::*;
pub mod lru;
use crate::lru::lru;
pub mod lfu;
use crate::lfu::lfu;
// pub mod mfu;
// use crate::mfu::mfu;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;


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

    let (page_tab, ref_tab, size) = load_data(filename, size);

    // println!("{:?}", page_tab);
    // println!("{:?}", ref_tab);

    println!("lru, page_fault: {:?}",lru(page_tab.clone(), ref_tab.clone(), size));
    println!("lfu, page_fault: {:?}",lfu(page_tab.clone(), ref_tab.clone(), size));
    // println!("mfu, page_fault: {:?}",mfu(page_tab.clone(), ref_tab.clone(), 3)); // works
    // uncorrectly!
}
