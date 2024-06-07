// use std::os::unix::process;
#![allow(dead_code)]
use std::process;
// use std::{fs, env};
// use std::io::{self, BufRead};
// use std::env;
// use std::process;

pub mod proc;
use crate::proc::Process;

pub mod fcfs;
use crate::fcfs::fcfs;
pub mod sjf;
use crate::sjf::sjf_w;
use crate::sjf::sjf_nw;
pub mod rr;
use crate::rr::round_robin;

fn import_data (data: &mut Vec<Process>) {
    // let data = &Vec<Process>::new();
    // data.push(Process::new(0, 3));
    // data.push(Process::new(1, 2));
    // data.push(Process::new(4, 7));
    // data.push(Process::new(6, 3));

    data.push(Process::new(0, 8));
    data.push(Process::new(3, 5));
    data.push(Process::new(5, 1));
    data.push(Process::new(12, 3));
    data.push(Process::new(19, 1));
    // process::exit(-1);
    // let file
    
}

fn main() {
    let mut data: Vec<Process> = Vec::new();
    import_data(&mut data);
    println!("avg response time for fcfs: {}", fcfs(data.clone()));
    println!("avg response time for sjf_nw: {}", sjf_nw(data.clone()));
    println!("avg response time for sjf_w: {}", sjf_w(data.clone()));

    process::exit(0);
}
