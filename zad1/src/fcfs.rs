use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;

// First-Come, First-Served (FCFS) scheduling algorithm
pub fn fcfs(mut dataset: Vec<Process>) -> f64 {
    let mut time: u64 = 0;
    
    // Sort processes by their arrival time
    dataset.sort_by_key(|process| process.arrival());

    // Run the scheduling loop until all processes are finished
    while !all_is_finished(&dataset) {
        // Check and deactivate processes that have arrived by the current time
        check_arrival(&mut dataset, time);
        
        // Activate the first inactive process if no process is currently active
        if dataset.iter().all(|process| process.state() != ProcessState::Active) {
            for process in dataset.iter_mut() {
                if process.state() == ProcessState::Inactive {
                    (*process).activate();
                    break;
                }
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
