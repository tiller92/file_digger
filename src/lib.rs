use std::fs::{metadata,read_dir};

pub fn search_for_name(name:String, path:String){
    // recursivly search through a computer file system macOS
    recursive_file_search(name,path);
}

pub fn recursive_file_search(name: String, path:String ) {
// if no next file/ folser then break
     let directory = match read_dir(path){
        Ok(directory) => directory,
        Err(e) => return println!("{:?} err at dir/file", e),
     };

    //item is a DirEntry Struct
    for item in directory{
        let dir = item.unwrap();
        let meta = metadata(dir.path());
        match meta {
            Ok(place) => { if place.is_file() {
                println!("is file {:?}", dir)
            }else{
                println!("folder {:?}", dir.path());
                let buff_path = dir.path();
                let str_path = buff_path.to_str().unwrap();
                let string_path:String = String::from(str_path); 
                let new_name = String::from(&name);
                println!("new path {:?}", str_path );
                recursive_file_search(new_name,string_path )
                }
            },
            Err(e) => println!("{:?},{:?} was not sure what to do here with this one",e, dir),
        }
}


}

