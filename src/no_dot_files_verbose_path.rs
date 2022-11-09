use crate::recursive_file_search;
use std::fs::{metadata,read_dir};
use std::path;
pub fn no_dot_files_verbose_path(query: String, path: String) -> recursive_file_search::Pretty {
  // need to ignore . files and print full path

    let mut file_count:u32 = 0;
    let mut folder_count:u32 = 0;
    
  
    let mut res:Vec<String> = Vec::new();
    let directory = match read_dir(path){
        Ok(directory) => directory,
        Err(_) =>  {
        let fail = recursive_file_search::Pretty {
                    found:res,
                    folders:0,
                    files:0,
                };   
        return fail    
        } , 
     };
    
    //item is a DirEntry Struct
    for item in directory{
        let dir = item.unwrap();
        let meta = metadata(dir.path());
        match meta {
            Ok(place) => {
                if place.is_dir() {
                    if dir.path().file_name().unwrap().to_str().unwrap().starts_with(".") ==false {
                        folder_count = folder_count +1;
                        println!(" {}", dir.path().to_str().unwrap());
                        let path_to_dir: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                        let query: path::PathBuf = path::PathBuf::from(&query);
                        if path_to_dir == query {
                            let item:String = String::from(dir.path().to_str().unwrap());
                            res.push(item)
                        }
                    }
                }
                 if place.is_file() { 
                    if dir.path().file_name().unwrap().to_str().unwrap().starts_with(".") ==false {
                        file_count = file_count +1;
                        println!("   »»»» {}", dir.path().file_name().unwrap().to_str().unwrap());
                        let location: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                        let query: path::PathBuf = path::PathBuf::from(&query);
                        if location == query {
                            let item:String = String::from(dir.path().to_str().unwrap());
                            res.push(item)
                    }
                     
                    }else{
                        // if current file turn into a string call pass it
                        let buff_path = dir.path();
                        let str_path = buff_path.to_str().unwrap();
                        let string_path:String = String::from(str_path); 
                        let new_query = String::from(&query);
                        let res2 = no_dot_files_verbose_path(new_query,string_path );
                            for file in res2.found {
                                res.push(file)
                            }
  
                        folder_count = folder_count + res2.folders;
                        file_count = file_count + res2.files;
                    }
                 }
            },
            Err(e) => println!("{},{} was not sure what to do here with this one" , 
                e,
                dir.path().to_str().unwrap())
        }
    }
    let res_pretty = recursive_file_search::Pretty {
        found: res,
        folders: folder_count,
        files:file_count,
    };
    res_pretty
  }


