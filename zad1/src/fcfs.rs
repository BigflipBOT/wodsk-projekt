use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;

pub fn fcfs(mut dataset: Vec<Process>) -> f64 { // TODO: what happens when there are two processes arriving ath the same time?
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
