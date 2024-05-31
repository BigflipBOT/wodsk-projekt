#[derive(Copy, Clone)]
pub struct Process {
    state: ProcessState,        //state of the process
    arrival_time: u64,          //when process arrives 
    length: u64,                //how much time is needet for completion 
    completion: u64,            //how complete is a process 
    standby_time: u64,          //how much process is waiting existing and not being completed
}
impl Process {
    pub fn new(arrival: u64, len: u64) -> Self {
        Process { state: ProcessState::Unborn, arrival_time: arrival, length: len, completion: 0, standby_time: 0, }
    }
    pub fn increment(&mut self) {
        if self.state == ProcessState::Active { // incresing completion on process when the process is in active state
            self.completion += 1;
            if self.completion >= self.length {
                self.state = ProcessState::Finished;
            }
        } else if self.state == ProcessState::Inactive { // and incresing the standby time when process spawned but is not active
            self.standby_time += 1;
        }
    }
    pub fn state(&self) -> ProcessState {
        return self.state;
    }
    pub fn activate(&mut self) {
        self.state = ProcessState::Active;
    }
    pub fn deactivate(&mut self) {
        self.state = ProcessState::Inactive;
    }
    pub fn arrival(&self) -> u64 {
        self.arrival_time
    }
    pub fn standby_time(&self) -> u64 { self.standby_time }
    pub fn length(&self) -> u64 { self.length }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ProcessState {
    Active,     // process is being completed
    Inactive,   // process spawned but is not being completed
    Finished,   // process is completed (finished)
    Unborn,     // process is not yet spawned 
}
