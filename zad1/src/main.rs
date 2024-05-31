#![allow(dead_code)]

#[derive(Copy, Clone)]
struct Process {
    state: ProcessState,        //state of the process
    arrival_time: u64,          //when process arrives in ms(?)
    length: u64,                //how much time is needet for completion in ms(?)
    completion: u64,            //how complete is a process in ms(?)
}
impl Process {
    fn new(arrival: u64, len: u64) -> Self {
        Process { state: ProcessState::Inactive, arrival_time: arrival, length: len, completion: 0 }
    }
    fn increment(&mut self) {
        self.completion += 1;
        if self.completion >= self.length {
            self.state = ProcessState::Finished;
        }
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum ProcessState {
    Active,
    Inactive,
    Finished,
}

fn is_all_finished(processes: &Vec<Process>) -> bool {
    for process in processes {
        if process.state != ProcessState::Finished {
            return false; 
        }
    }
    return true;
}

fn import_data (data: &mut Vec<Process>) {
    // data = &Vec<process>::new();
    data.push(Process::new(0, 3));
    data.push(Process::new(1, 2));
    data.push(Process::new(4, 7));
    data.push(Process::new(6, 3));
}

fn round_robin(dataset: Vec<Process>) {
    unimplemented!();
}

fn fcfs(mut dataset: Vec<Process>) {
    unimplemented!();
    // println!("dataset: {:?}", dataset[0].state);
    // let mut process_que: &mut Vec<Process> = &mut Vec::new();
    // let mut process_que = vec![&dataset[0]];
    // process_que.push(dataset[0]);
    // (*process_que)[0].state = ProcessState::Finished;
    // println!("dataset: {:?}, process_que: {:?}", dataset[0].state, process_que[0].state);
    // while is_all_finished(&dataset) {
        // unimplemented!()
    // }
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
    fcfs(data);
}
