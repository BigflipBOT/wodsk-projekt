use u64 as IdType; // for clarity
use crate::memory::Page;
use crate::memory::MemSim;

// Function to find the least recently used page
fn find_lest_recent_usage(pages_table: Vec<Page> ) -> u64 {
    if pages_table.len() == 0 {
        return 0;
    }
    let index = 
        pages_table.iter().min_by_key(|page| page.get_recent_usage());

    return index.unwrap().get_id();
}

// Function to simulate the Least Recently Used (LRU) page replacement algorithm
pub fn lru(mut pages_template: Vec<Page>, page_usage_order: Vec<IdType>, max_capacity: usize) -> u64 {

    let mut memory_managment = MemSim::new(max_capacity);

    for page_id in page_usage_order {
        let next_to_swap_index: IdType = find_lest_recent_usage(memory_managment.get_page_list(pages_template.clone()));

        // Increment memory management and potentially swap pages
        memory_managment.increment(/*pages_template.clone(),*/  page_id, next_to_swap_index);

        // Update the usage information for the accessed page
        for page in &mut pages_template {
            if page.get_id() == page_id {
                page.use_page(memory_managment.get_step() - 1);
                break;
            }
        }
        // End of the loop sequence
    }

    // Return the number of page faults that occurred
    memory_managment.get_page_fault()
}

