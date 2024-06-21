use u64 as IdType; // for clarity
use u64 as StepType;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    page_id: IdType, // id (name of the page). there can only be one page with given id
    page_usage: u64, // how much was this page used
    page_recent_u: StepType, // when was this page last used
}

impl Page {
    // Constructor for creating a new page
    pub fn new(id: IdType) -> Self {
        Page{ page_id: id, page_usage: 0, page_recent_u: 0 }
    }

    // Get the ID of the page
    pub fn get_id(&self) -> IdType {
        self.page_id
    }

    // Get the most recent usage step of the page
    pub fn get_recent_usage(&self) -> u64 {
        self.page_recent_u
    }

    // Get the total usage count of the page
    pub fn get_usage(&self) -> u64 {
        self.page_usage
    }

    // Update the page's usage information
    pub fn use_page(&mut self, curr_step: StepType) {
        self.page_recent_u = curr_step; // set last usage time
        self.page_usage = self.page_usage + 1; // increment usage
    }
}

pub struct MemSim {
    page_id_list: Vec<IdType>, // list of page IDs currently in memory
    page_fault: u64, // count of page faults
    step: StepType, // current simulation step
    max_capacity: usize, // maximum number of pages that can be held in memory
}

impl MemSim {
    // Constructor for creating a new memory simulator
    pub fn new( max_cap: usize, ) -> Self {
        MemSim { page_id_list: Vec::new(), page_fault: 0, step: 0, max_capacity: max_cap  }
    }

    // Get the list of pages currently in memory
    pub fn get_page_list(&self, full_list: Vec<Page>) -> Vec<Page> {
        full_list.into_iter()
            .filter(|page| self.page_id_list.contains(&page.page_id))
            .collect()
    }

    // Get the current simulation step
    pub fn get_step(&self) -> StepType {
        self.step
    } 

    // Get the number of page faults
    pub fn get_page_fault(&self) -> u64 {
        self.page_fault
    }

    // Increment the page fault count
    fn page_fault(&mut self) {
        self.page_fault = self.page_fault+1;
    }

    // Check if a page is missing from memory
    fn check_missing(&self, needed_id: IdType) -> bool {
        // Checks if the self, contains checked value.
        if self.page_id_list.iter().find(|value| **value == needed_id).is_some() {
            // print!("check_missing false");
            return false
        }
        return true
    }

    // Increment the simulation step and potentially handle a page fault
    pub fn increment(&mut self, /*_full_page_table: RefCell<Vec<Page>>,*/ needed: IdType, next_to_swap: IdType) {
        // print!("{0} || {next_to_swap} || {needed} || ", self.step);
        // checking if there is a needed page already loaded
        if self.check_missing(needed) {
            self.page_fault();
            // println!("needed: {needed} | missing: {needed}");

            // loading in page when there is space
            if self.page_id_list.len() < self.max_capacity {
                self.page_id_list.push(needed)
            }
            else { // and when there is no space left:
                // swapping 'next_to_swap' for 'needed'
                for id in &mut self.page_id_list {
                    // print!("| {id} ");
                    if *id == next_to_swap {
                        if next_to_swap == 0 { panic!("next_to_swap == 0"); }
                        *id = needed;
                        break;
                    }
                }
            }
        }
        else {
            // println!("needed: {needed} | missing: _");
        }
        // println!("");
        // let _ = self.page_id_list.iter().for_each(|x| print!("{} ",x));

        if self.page_id_list.len() > self.max_capacity {
            panic!("length exceeded max_capacity");
        }
        
        // simulating time passage
        self.step = self.step + 1;
    }
} 

