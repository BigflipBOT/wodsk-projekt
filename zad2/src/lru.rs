use crate::memory::Page;
use crate::memory::MemSim;

fn find_lest_recent_usage(pages_table: Vec<Page> ) -> u64 {
    let index: u64 = 
        (pages_table.iter().min_by_key(|page| *page.get_recent_usage())).get_id();

    return index;
}

fn lru(pages_table: Vec<Page>, page_usage_order: Vec<Page>, max_capacity: u64) {

    let mut memory_managment = MemSim::new(max_capacity);

    for page in &page_usage_order {
        let next_index: u64;
        // unimplemented!();

        // memory_managment.increment(unimplemented!());
    }
}
