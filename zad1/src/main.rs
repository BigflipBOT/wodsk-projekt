#![allow(dead_code)]
pub mod proc;

use std::u64;

use crate::proc::Process;
use crate::proc::ProcessState;
    
fn all_is_finished<'a>(processes: &'a Vec<Process>) -> bool {
    processes.iter().all(|process| process.state() == ProcessState::Finished)
    // for process in processes {
    //     if process.state() != ProcessState::Finished {
    //         return false; 
    //     }
    // }
    // return true;
}

fn check_arrival(processes: &mut Vec<Process>, cur_time: u64) {
    for process in processes.iter_mut() {
        if process.state() == ProcessState::Unborn {
            if process.arrival() <= cur_time {
                (*process).deactivate();
            }
        }
    }
}

fn import_data (data: &mut Vec<Process>) {
    // data = &Vec<process>::new();
    data.push(Process::new(0, 3));
    data.push(Process::new(1, 2));
    data.push(Process::new(4, 7));
    data.push(Process::new(6, 3));
}

fn round_robin(mut dataset: Vec<Process>) -> u64 {
    let mut time: u64 = 0;

    let kwant: u64 = 2;
    let mut chamber_time: u64 = 0;
    let mut counter: usize = 0;

    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            for process in dataset.iter_mut() {
                if process.state() == ProcessState::Inactive {
                    println!("no active process, activating first process");
                    (*process).activate();
                    break;
                }
            }
        }
        if chamber_time >= kwant {
            unimplemented!()
        }
        //
        time += 1;
    }

    unimplemented!();
}

fn fcfs(mut dataset: Vec<Process>) -> u64 {
    let mut time: u64 = 0;

    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm:
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            for process in dataset.iter_mut() {
                if process.state() == ProcessState::Inactive {
                    println!("no active process, activating first process");
                    (*process).activate();
                    break;
                }
            }
        }

        for process in dataset.iter_mut() {
            if process.state() == ProcessState::Active {
                println!("incrementing process");
                (*process).increment();
            }
        }

        //debug things and stuff below 
        // println!("time: {:?}", time);
        // for process in &dataset {
        //     println!("{:?}", process.state())
        // }
        time += 1;
    }
    time / (dataset.len() as u64)
}

fn sjf_nw() {
    unimplemented!();
}

fn sjf_w() {
    unimplemented!();
}

fn main() {
    let mut data: Vec<Process> = Vec::new();
    import_data(&mut data);
    // println!("{:?}", data);
    println!("avg response time for fcfs: {}", fcfs(data));
}
