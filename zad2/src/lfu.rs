use u64 as IdType; // for clarity
use crate::memory::Page;
use crate::memory::MemSim;

// Function to find the least frequently used page
fn find_lest_frequent_usage(mut pages_table: Vec<Page> ) -> IdType {
    if pages_table.len() == 0 {
        return 0;
    }
    pages_table.sort_by_key(|page| page.get_recent_usage());
    // pages_table.iter().for_each(|&x| print!("{} |", x.get_id()));
    // println!("");
    if let Some(x) = pages_table.iter().min_by_key(|page| page.get_usage()) {
        // println!("{:?}", x);
        return x.get_id();
    }
    else {
        return pages_table[0].get_id();
    }
}

// Function to simulate the Least Frequently Used (LFU) page replacement algorithm
pub fn lfu(mut pages_template: Vec<Page>, page_usage_order: Vec<IdType>, max_capacity: usize) -> u64 {

    let mut memory_managment = MemSim::new(max_capacity);

    for page_id in page_usage_order {
        let next_to_swap_index: IdType = find_lest_frequent_usage(memory_managment.get_page_list(pages_template.clone()));

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

