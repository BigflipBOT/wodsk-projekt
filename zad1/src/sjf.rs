use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;

// Shortest Job First - Non-Preemptive (SJF-NW) scheduling algorithm
pub fn sjf_nw(mut dataset: Vec<Process>) -> f64 {
    let mut time: u64 = 0;

    // Run the scheduling loop until all processes are finished
    while !all_is_finished(&dataset) {
        // Check and deactivate processes that have arrived by the current time
        check_arrival(&mut dataset, time);
        
        // Activate the shortest inactive process if no process is currently active
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            if let Some(process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Inactive).min_by_key(|process| process.length()) {
                process.activate();
            }
        }

        // Increment the state of all processes (either completion or standby time)
        for process in dataset.iter_mut() {
            (*process).increment();
        }
        
        // Increment the global time
        time += 1;
    }
    
    // Calculate and return the average waiting time of all processes
    avg_wait_time(&dataset)    
}

// Shortest Job First - Preemptive (SJF-W) scheduling algorithm
pub fn sjf_w(mut dataset: Vec<Process>) -> f64 {
   let mut time: u64 = 0;

    // Run the scheduling loop until all processes are finished
    while !all_is_finished(&dataset) {
        // Check and deactivate processes that have arrived by the current time
        check_arrival(&mut dataset, time);
        
        // Deactivate the currently active process, if any
        if let Some(active_process) = dataset.iter_mut().find(|process| process.state() == ProcessState::Active) {
            active_process.deactivate();
        }
        
        // Activate the process with the shortest time left if no process is currently active
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            if let Some(process) = dataset.iter_mut().filter(|process| process.state() == ProcessState::Inactive).min_by_key(|process| process.time_left()) {
                process.activate();
            }
        }

        // Increment the state of all processes (either completion or standby time)
        for process in dataset.iter_mut() {
            (*process).increment();
        }
        
        // Increment the global time
        time += 1;
    }
    
    // Calculate and return the average waiting time of all processes
    avg_wait_time(&dataset)
}

