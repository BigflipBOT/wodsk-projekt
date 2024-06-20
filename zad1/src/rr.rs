use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;

use std::cell::RefCell;

pub fn round_robin(mut dataset: Vec<Process>) -> f64 {
    unimplemented!();
    let mut time: u64 = 0;


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

    avg_wait_time(&dataset)
}

