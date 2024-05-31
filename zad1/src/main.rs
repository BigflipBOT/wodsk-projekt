#![allow(dead_code)]
pub mod proc;
use crate::proc::Process;
use crate::proc::ProcessState;

use std::cell::RefCell;
use std::usize;
    
fn all_is_finished<'a>(processes: &'a Vec<Process>) -> bool {
    processes.iter().all(|process| process.state() == ProcessState::Finished)
    // for process in processes {
    //     if process.state() != ProcessState::Finished {
    //         return false; 
    //     }
    // }
    // return true;
}

fn check_arrival(processes: &mut Vec<Process>, cur_time: u64) -> usize {
    let mut counter: usize = 0;
    for process in processes.iter_mut() {
        if process.state() == ProcessState::Unborn {
            if process.arrival() <= cur_time {
                (*process).deactivate();
                counter += 1;
            }
        }
    }
    counter
}

fn avg_wait_time(processes: &Vec<Process>) -> f64 {
    let mut sum: f64 = 0.0;
    for process in processes.iter() {
        sum += process.standby_time() as f64;
    }
    sum / (processes.len() as f64)
}

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
}

fn round_robin(mut dataset: Vec<Process>) -> u64 {
    let mut time: u64 = 0;

    let mut new_process_counter: usize = 0;
    let kwant: u64 = 2;
    let mut chamber_time: u64 = 0;
    let mut counter: usize = 0;

    dataset.push(Process::new(0, 8));
    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm
        let mut process_que: Vec<RefCell<Process>> = Vec::new(); 
        for process in dataset.iter_mut() {
            if process.state() == ProcessState::Inactive { process_que.push(RefCell::new(*process)) }
        }

        while process_que.iter().all(|process| (process.borrow()).state() != ProcessState::Finished) {
            let mut iterations = 0;
            process_que[iterations].borrow_mut().increment();
        }
        //
        time += 1;
    }

    unimplemented!();
}

fn fcfs(mut dataset: Vec<Process>) -> f64 { // TODO: what happens when there are two processes arriving ath the same time?
    let mut time: u64 = 0;
    dataset.sort_by_key(|process| process.arrival());

    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm:
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            for process in dataset.iter_mut() {
                if process.state() == ProcessState::Inactive {
                    // println!("no active process, activating first process");
                    (*process).activate();
                    break;
                }
            }
        }

        for process in dataset.iter_mut() {
            // println!("incrementing process");
            (*process).increment();
        }

        //debug things and stuff below 
        // println!("time: {:?}", time);
        // for process in &dataset {
        //     println!("{:?}", process.state())
        // }
        time += 1;
    }
    avg_wait_time(&dataset)
}

fn sjf_nw(mut dataset: Vec<Process>) -> f64 {
    let mut time: u64 = 0;

    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            if let Some(process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Inactive).min_by_key(|process| process.length()) {
                process.activate();
            }
        }    

        for process in dataset.iter_mut() {
            (*process).increment();
        }
        time += 1;
    }
    avg_wait_time(&dataset)    
}

fn sjf_w(mut dataset: Vec<Process>) -> f64 { // TOdo ask if alghoritm is corect (about weightsystem)
   let mut time: u64 = 0;

    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm
        // if let Some(shortest_process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Inactive).min_by_key(|process| process.length()) {
        //     // change the state of the shortest inactive process to activa                                           this code below (.min_by_key(...)) is kind of hacky solltution to keep compiler happy
        //     if let Some(active_process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Active).min_by_key(|process| process.length()) {
        //         // change the state of the shortest inactive process to active
        //         active_process.activate();
        //     }
        //     shortest_process.activate();
        // }
        if let Some(active_process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Active).min_by_key(|proc| proc.length()) {
            active_process.deactivate();
        }
        if dataset.iter().all(|process| process.state() != ProcessState::Active) { // the same alghoritm as for sjf_nw
            if let Some(process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Inactive).min_by_key(|process| process.length()) {
                process.activate();
            }
        }    
        //if the currently active alghoritm was shortest nothing changes after above 8 lines


        for process in dataset.iter_mut() {
            (*process).increment();
        }
        time += 1;
    }
    avg_wait_time(&dataset)
}


fn main() {
    let mut data: Vec<Process> = Vec::new();
    import_data(&mut data);
    println!("avg response time for fcfs: {}", fcfs(data.clone()));
    println!("avg response time for sjf_nw: {}", sjf_nw(data.clone()));
    println!("avg response time for sjf_w: {}", sjf_w(data.clone()));

}
