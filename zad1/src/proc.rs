#[derive(Copy, Clone)]
pub struct Process {
    state: ProcessState,        //state of the process
    arrival_time: u64,          //when process arrives in ms(?)
    length: u64,                //how much time is needet for completion in ms(?)
    completion: u64,            //how complete is a process in ms(?)
}
impl Process {
    pub fn new(arrival: u64, len: u64) -> Self {
        Process { state: ProcessState::Unborn, arrival_time: arrival, length: len, completion: 0 }
    }
    pub fn increment(&mut self) {
        self.completion += 1;
        if self.completion >= self.length {
            self.state = ProcessState::Finished;
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
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ProcessState {
    Active,
    Inactive,
    Finished,
    Unborn,
}

