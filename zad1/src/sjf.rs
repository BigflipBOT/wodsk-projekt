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

pub fn sjf_w(mut dataset: Vec<Process>) -> f64 {
   let mut time: u64 = 0;

    while !all_is_finished(&dataset) {
        check_arrival(&mut dataset, time);
        
        //alghoritm
        // find (only one exists) active process and deactivate it
        if let Some(active_process) = dataset.iter_mut().find(|process| process.state() == ProcessState::Active) {
            active_process.deactivate();
        }
        // find process with lowest time left to completion and activate it
        if dataset.iter().all(|process| process.state() != ProcessState::Active) { // the same alghoritm as for sjf_nw
            if let Some(process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Inactive).min_by_key(|process| process.time_left()) {
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
