use crate::proc::Process;
use crate::proc::ProcessState;
use crate::proc::all_is_finished;
use crate::proc::check_arrival;
use crate::proc::avg_wait_time;
use crate::proc::processor_tick;

use std::cell::RefCell;
use std::usize;

pub fn round_robin(mut dataset: Vec<Process>) -> f64 {
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

        for process in process_que.iter_mut() {
            println!("some process activated");
            process.borrow_mut().activate();
            for _i in 0..kwant {
               if process.borrow().state() == ProcessState::Finished {
                   println!("process finished before time limit");
                   break;
               }
               processor_tick(&mut dataset, &mut time);
               println!("{}", time);
            }
            process.borrow_mut().deactivate();
        }
    }
    
    avg_wait_time(&dataset)
}

