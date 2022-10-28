use file_digger;
use std::{env,process};

fn main() { 
    // this should call a function that takes a string and searches through the file system
   let args = env::args();
   let config = file_digger::Config::build(args);
   println!("{:?}",config);
   let name: String = String::from("help");
   let path: String = String::from("/Users/ryantiller/Documents/rust-fun/minigrep/output.txt");
   
   file_digger::search_for_name(config)
}
