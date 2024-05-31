use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;

pub fn sjf_nw(mut dataset: Vec<Process>) -> f64 {
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

pub fn sjf_w(mut dataset: Vec<Process>) -> f64 { // TOdo ask if alghoritm is corect (about weightsystem)
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