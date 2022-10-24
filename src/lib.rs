use std::fs::read_dir;
use std::io;

pub fn search_for_name(name: String){
    println!("{}", name);
    // recursivly search through a computer file system macOS
    recursive_file_search();
}

pub fn recursive_file_search() {
// if no next file/ folser then break
//
// // normal condition should just print file name and then call the function again 
    let mut read = read_dir("../..").expect("no such file");
    for dir in read {
        println!("{:?}", dir);
}

}

