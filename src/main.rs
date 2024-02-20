use std::{fs::File, io::BufReader};
use sorts::merge_sort;
use serde_json;

pub mod sorts;

fn main() {
    let path = "./nums.json";
    let file = File::open(path).expect("unable to read file");
    let reader = BufReader::new(file);
    let loaded_nums: Vec<u32> = serde_json::from_reader(reader).expect("unable to read array");
    
    let nums = (1u32..10001).collect::<Vec<_>>();
    
    let loaded_nums = sorts::merge_sort(&loaded_nums);
    let nums = merge_sort(&nums);
    
    println!("{}", loaded_nums.eq(&nums));
}