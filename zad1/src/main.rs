use std::process;

pub mod proc;
use crate::proc::Process;

pub mod fcfs;
use crate::fcfs::fcfs;
pub mod sjf;
use crate::sjf::sjf_w;
use crate::sjf::sjf_nw;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn import_data(data: &mut Vec<Process>, file_name: &str) -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    
    // Create a new buffered reader to read the file line by line.
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file.
    for line in reader.lines() {
        let line = line?;
        // Split the line by semicolons to get each pair.
        for pair in line.split(";") {
            // Ignore empty pairs (e.g., due to trailing semicolon)
            if pair.trim().is_empty() {
                continue;
            }
            // Split each pair by comma to get the two numbers.
            let numbers: Vec<&str> = pair.split(",").collect();
            if numbers.len() == 2 {
                // Parse the numbers and create a new Process.
                if let (Ok(arrival_time), Ok(length)) = (numbers[0].trim().parse::<u64>(), numbers[1].trim().parse::<u64>()) {
                    let process = Process::new(arrival_time, length);
                    // Add the new process to the data vector.
                    data.push(process);
                }
            }
        }
    }
    
    Ok(())
}

fn main() {
    let mut data: Vec<Process> = Vec::new();

    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();
    
    // Check if the correct number of arguments is provided.
    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    
    match import_data(&mut data, file_name) {
        Ok(_) => {
            println!("data loaded correctly.");
        }
        Err(e) => eprintln!("Failed to import data: {}", e),
    }

    // run every alghoritm and display test data
    println!("avg response time for fcfs: {}", fcfs(data.clone()));
    println!("avg response time for sjf_nw: {}", sjf_nw(data.clone()));
    println!("avg response time for sjf_w: {}", sjf_w(data.clone()));

    process::exit(0);
}
