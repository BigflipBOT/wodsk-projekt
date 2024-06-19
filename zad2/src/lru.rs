use u64 as IdType; // for clarity
use crate::memory::Page;
use crate::memory::MemSim;
use std::cell::RefCell;

fn find_lest_recent_usage(pages_table: Vec<Page> ) -> u64 {
    if pages_table.len() == 0 {
        return 0;
    }
    let index = 
        pages_table.iter().min_by_key(|page| page.get_recent_usage());

    return index.unwrap().get_id();
}

pub fn lru(mut pages_template: Vec<Page>, page_usage_order: Vec<IdType>, max_capacity: u64) -> u64 {

    let mut memory_managment = MemSim::new(max_capacity);

    for page_id in page_usage_order {
        let next_to_swap_index: IdType = find_lest_recent_usage(memory_managment.get_page_list(pages_template.clone()));
        // let next_to_swap_index: IdType = find_lest_recent_usage(
        //         memory_managment.get_page_list(
        //             pages_template.borrow().to_vec()
        //         )
        //     );
        // pages_template.into_iter().filter(|page| self.page_id_list.contains(&page.page_id)).collect()
        
        // from here
        memory_managment.increment(/*pages_template.clone(),*/  page_id, next_to_swap_index);

        for page in &mut pages_template {
            if page.get_id() == page_id {
                page.use_page(memory_managment.get_step() - 1);
                break;
            }
        }
        //to here (this is an closing sequence for every loop invocation)

        // memory_managment.increment(pages_table[pages_table.iter().position(|val| val.borrow().get_id() == *page_id).unwrap()], next_to_swap_index);
    }

    memory_managment.get_page_fault()
}
