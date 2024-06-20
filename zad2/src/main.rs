#![allow(dead_code)]
pub mod memory;
use u64 as IdType;
use crate::memory::Page;
// use crate::memory::MemSim;
// use std::cell::RefCell;

pub mod fifo;
use crate::fifo::*;
pub mod lru;
use crate::lru::lru;
pub mod lfu;
use crate::lfu::lfu;
pub mod mfu;
use crate::mfu::mfu;

fn load_data() -> (Vec<Page>, Vec<IdType>){
    let reference_table: Vec<IdType> = vec![1,2,3,4,1,2,5,1,2,3,4,5]; // kolejność odwołań
    let mut pages: Vec<Page> = Vec::new();
    for id in & reference_table{
        if !(pages.iter().find(|value| value.get_id() == *id).is_some()) {
            pages.push(Page::new(*id));
        }
    }

    return (pages, reference_table);
}

fn main() {
    let (page_tab, ref_tab) = load_data();
    // println!("{:?}", page_tab);
    // println!("{:?}", ref_tab);

    println!("lru, page_fault: {:?}",lru(page_tab.clone(), ref_tab.clone(), 3));
    println!("lfu, page_fault: {:?}",lfu(page_tab.clone(), ref_tab.clone(), 3));
    // println!("mfu, page_fault: {:?}",mfu(page_tab.clone(), ref_tab.clone(), 3)); // works
    // uncorrectly!
}
