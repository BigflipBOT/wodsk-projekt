use u64 as IdType; // for clarity
use std::cell::RefCell;
use std::cell::Ref;

#[derive(Debug)]
pub struct Page {
    page_id: IdType, // id (name of the page). there can only be one page with given id
    page_usage: u64, // how much was this page used
    page_recent_u: u64, // when was this page last used
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
}

pub struct MemSim {
    page_list: Vec<RefCell<Page>>, // vec is easier thatn array cuz it can be empty :heareyes:
    // needed: IdType, // zero will be treaded as null
    page_fault: IdType,
    step: u64,
    max_capacity: u64,
}
impl MemSim {
    pub fn new( max_cap: u64, ) -> Self {
        MemSim { page_list: Vec::new(), page_fault: 0, step: 0, max_capacity: max_cap  }
    }
    fn page_fault(&mut self) {
        self.page_fault = self.page_fault+1;
    }
    fn check_missing(&mut self, needed: Ref<Page>) -> bool {
        //bellow, checks if the self, contains checked value.
        if self.page_list.iter().find(|value| value.borrow().page_id == needed.page_id).is_some() {
            return false
        }
        self.page_fault();
        return true
    }
    pub fn increment(&mut self, needed: IdType, next_to_swap: IdType) {
        // i can only compare ID's, no need to pass pointers to the whole objects!
        // kind of makes things easier than what i tried before...
        unimplemented!();
    }
    // pub fn increment(&mut self, needed: RefCell<Page>, next: IdType) {
    //     // next field is the next thing that should be swapped if needed
    //     // if there is nothing missing, "next" parameter, should be ignored
    //     let is_missing = self.check_missing(needed.borrow());
    //     if is_missing {
    //         if let Some(index) = self.page_list.iter().position(|value| value.borrow().page_id == next){ //this is kind of unsafe cuz when there is no next, program crashes (?)
    //             // self.page_list.swap_remove(index); // release space for next page (replacing one with other is hard, and order doesn't matter)
    //             // self.page_list.push(needed)
    //             self.page_list[index] = needed; //is it too good to be true?
    //         }                                                                            
    //     }
    // }
}
