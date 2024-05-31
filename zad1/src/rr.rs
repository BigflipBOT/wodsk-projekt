use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;

use std::cell::RefCell;
use std::usize;

pub fn round_robin(mut dataset: Vec<Process>) -> u64 {
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

