use file_digger;
use std::{env,process};

fn main() { 
    // this should call a function that takes a string and searches through the file system
   let args = env::args();
   let name: String = String::from("help");
   let path: String = String::from("/Users/ryantiller/Documents/rust_projects");
   file_digger::search_for_name(name,path);
}
