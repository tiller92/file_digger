use std::fs::{metadata,read_dir};

pub fn search_for_name(name: String){
    println!("{}", name);
    // recursivly search through a computer file system macOS
    recursive_file_search(name);
}

pub fn recursive_file_search(_name: String) {
// if no next file/ folser then break
//
// // normal condition should just print file name and then call the function again
// will call find_root which tells us where to begin our search
    let path = metadata("/");
    println!("hope this works {:?}", path);
    //folder or path you want to start at
    let directory = read_dir("/").expect(" make sure you give a path to a folder");
    
// here loop through each folder or file and if a folder call recusive file search for all file
// print for now
    for item in directory {
        let meta = metadata(item.unwrap().path());
            println!("{:?}", &meta.unwrap().is_file());
            //if meta.unwrap().is_file()
        //println!("item, {:?}", item.unwrap().path());    
}
}


