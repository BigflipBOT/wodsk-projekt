use u64 as IdType; // for clarity
use u64 as StepType;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    page_id: IdType, // id (name of the page). there can only be one page with given id
    page_usage: u64, // how much was this page used
    page_recent_u: StepType, // when was this page last used
}
impl Page {
    pub fn new(id: IdType) -> Self {
        Page{ page_id: id, page_usage: 0, page_recent_u: 0 }
    }
    pub fn get_id(&self) -> IdType {
        self.page_id
    }
    pub fn get_recent_usage(&self) -> u64 {
        self.page_recent_u
    }
    pub fn get_usage(&self) -> u64 {
        self.page_usage
    }
    pub fn use_page(&mut self, curr_step: StepType) {
        self.page_recent_u = curr_step; // set last usage time
        self.page_usage = self.page_usage + 1; // increment usage
    }
}

pub struct MemSim {
    page_id_list: Vec<IdType>,
    page_fault: u64,
    step: StepType,
    max_capacity: usize,
}
impl MemSim {
    pub fn new( max_cap: usize, ) -> Self {
        MemSim { page_id_list: Vec::new(), page_fault: 0, step: 0, max_capacity: max_cap  }
    }
    pub fn get_page_list(&self, full_list: Vec<Page>) -> Vec<Page> {
        full_list.into_iter()
            .filter(|page| self.page_id_list.contains(&page.page_id))
            .collect()
    }
    pub fn get_step(&self) -> StepType {
        self.step
    } 
    pub fn get_page_fault(&self) -> u64 {
        self.page_fault
    }
    fn page_fault(&mut self) {
        self.page_fault = self.page_fault+1;
    }
    fn check_missing(&self, needed_id: IdType) -> bool {
        //bellow, checks if the self, contains checked value.
        if self.page_id_list.iter().find(|value| **value == needed_id).is_some() {
            // print!("check_missing false");
            return false
        }
        return true
    }
    pub fn increment(&mut self, /*_full_page_table: RefCell<Vec<Page>>,*/ needed: IdType, next_to_swap: IdType) {
        // print!("{0} || {next_to_swap} || {needed} || ", self.step);
        // checking if there is a needed page already loaded
        if self.check_missing(needed) {
            self.page_fault();
            println!("needed: {needed} | missing: {needed}");

            // loading in page when there is space
            if self.page_id_list.len() < self.max_capacity {
                self.page_id_list.push(needed)
            }
            else { // and when there is no space left:
                // swapping 'next_to_swap' for 'needed'
                for sex in &mut self.page_id_list {
                    // print!("| {sex} ");
                    if *sex == next_to_swap {
                        if next_to_swap == 0 { panic!("next_to_swap == 0"); }
                        *sex = needed;
                        break;
                    }
                }
            }
        }
        else {
            println!("needed: {needed} | missing: _");
        }
        // println!("");
        let _ = self.page_id_list.iter().for_each(|x| print!("{} ",x));

        if self.page_id_list.len() > self.max_capacity {
            panic!("lenght exceeded max_capacity");
        }
        
        // full_page_table.borrow_mut().iter().find(|page| page.page_id == needed).unwrap
        
        // unimplemented!();// page.use_page() goes here! // or not. can be used in main functuin
        // for esaier implementation
        
        // simulating time passage
        self.step = self.step + 1;
    }
} 
