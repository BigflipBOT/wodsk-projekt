use u64 as IdType; // for clarity
use crate::memory::Page;
use crate::memory::MemSim;

fn find_most_frequent_usage(mut pages_table: Vec<Page> ) -> IdType {
    if pages_table.len() == 0 {
        return 0;
    }
    pages_table.sort_by_key(|page| -(page.get_recent_usage()as i64));
    // pages_table.iter().for_each(|&x| print!("{} |", x.get_id()));
    // println!("");
    if let Some(x) = pages_table.iter().max_by_key(|page| page.get_usage()) {
        // println!("{:?}", x);
        return x.get_id();
    }
    else {
        return pages_table[0].get_id();
    }
}

pub fn mfu(mut pages_template: Vec<Page>, page_usage_order: Vec<IdType>, max_capacity: usize) -> u64 {

    let mut memory_managment = MemSim::new(max_capacity);

    for page_id in page_usage_order {
        let next_to_swap_index: IdType = find_most_frequent_usage(memory_managment.get_page_list(pages_template.clone()));

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
