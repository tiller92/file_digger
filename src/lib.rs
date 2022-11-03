use std::fs::{metadata,read_dir};
use std::path;

#[derive(Debug)]
pub struct Config {
    pub local_path:String,
    pub path:String,
    pub query:String, 
}

impl Config {
    // handle all command line interface in build pass a to recursice func type config
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        //should always be current path
       let local_path = match args.next(){
            Some(path) => path,
            None => String::from(" ") 
       };
        // confirms if user gave a folder/file path
       let mut user_path:String = String::from(&local_path);
       let string_path_id = "/";
       let path = match args.next(){
            Some(path)=>path,
            None => String::from(&local_path),
       };
       println!("{}", path);
       if path.contains(string_path_id){
            user_path = path.clone();
       }

       // check for a string that doesnt start with a '-' that will be out query
        let args_after_two:Vec<String> = args.collect();
          for arg in args_after_two {
              if arg.starts_with("-"){
                //eventually add more functionality with flags
              }else if arg.contains("/"){
                println!("another path?")
              }
            println!("arg:  {:?}", arg);
       }  
       Ok(Config{
           path:user_path,
           query:String::from("no query"),
           local_path})
    }
}

// struct that hold the res of the cooler (pretty) recurse function
pub struct Pretty {
    found: Vec<String>,
    folders: u32,
    files:u32,
}

// handle arg length and finalize funcionality
pub fn handle_args(config:Result<Config, &'static str>){ 
    //fn really just handles the config struct and handles errors before we start recursing 
    let config = match config {
        Ok(config) => config,
        _ =>  Config{local_path:String::from("/"),
                    path:String::from("/"),
                    query:String::from(" "),
                    } 
    };
    println!("{:?}", config);
    let user_query:String = String::from(&config.query); 
    let res = recursive_file_search(config.query,config.path);
    if res.found.len() > 0 {
        for item in res.found {
            println!("{}", item);
        }
    }else {
        println!(" No file or Directory with the name » '{}'", user_query) 
    }
    println!(" folders {}, files {} ", res.folders, res.files);
}

pub fn recursive_file_search(name: String, path:String ) -> Pretty{
    // Slow hopefully cool looking verions 
    // need to add counts for files and folders
    let mut file_count:u32 = 0;
    let mut folder_count:u32 = 0;
    

    let mut res:Vec<String> = Vec::new();
    let directory = match read_dir(path){
        Ok(directory) => directory,
        Err(_e) =>  {
        let fail = Pretty {
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
                    folder_count = folder_count +1;
                    println!(" {}", dir.path().to_str().unwrap());
                    let path_to_dir: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                    let query: path::PathBuf = path::PathBuf::from(&name);
                    if path_to_dir == query {
                        let item:String = String::from(dir.path().to_str().unwrap());
                        res.push(item)
                        }
                    }
                 if place.is_file() { 
                    file_count = file_count +1;
                    println!("   »»»» {}", dir.path().file_name().unwrap().to_str().unwrap());
                    let location: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                    let query: path::PathBuf = path::PathBuf::from(&name);
                        if location == query {
                            let item:String = String::from(dir.path().to_str().unwrap());
                            res.push(item)
                    }
            
                }else{
                    // if current file turn into a string call pass it
                    let buff_path = dir.path();
                    let str_path = buff_path.to_str().unwrap();
                    let string_path:String = String::from(str_path); 
                    let new_name = String::from(&name);
                    let res2 = recursive_file_search(new_name,string_path );
                        for file in res2.found {
                            res.push(file)
                        }

                    folder_count = folder_count + res2.folders;
                    file_count = file_count + res2.files;
                    }
            },
            Err(e) => println!("{:?},{:?} was not sure what to do here with this one",e, dir),
        }
    }
    let res_pretty = Pretty {
        found: res,
        folders: folder_count,
        files:file_count,
    };
    res_pretty
}
