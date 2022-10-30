use std::fs::{metadata,read_dir};
use std::path;

#[derive(Debug)]
// make a fast and pretty recurse, bool value passes to handle args in the config
pub struct Config {
 pub path:String,
 pub query:String, 
}

impl Config {
    // handle all command line interface in build pass a to recursice func type config
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("You must add a path so I know where to start digging"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query so just count all folder and files"), 
        };
       Ok(Config{path, query})
    }

}
// handle arg length and finalize funcionality
pub fn handle_args(config:Result<Config,&'static str>){ 
    //fn really just handles the config struct and handles errors before we start recursing 
   let config = match config {
        Ok(config) => config,
        _ => Config{path:String::from("/"), query:String::from(" ") }, 
   }; 
    let res = recursive_file_search(config.query,config.path);
    println!("{:?}", res)
}

pub fn recursive_file_search(name: String, path:String ) -> Vec<String>{
    // prettier verison that is slow but cool returns the number of folders and files searched and 
    // the ends with a vec of folders an files with the query(name)
   //Direcotory is an iterator that contains all the files or folders on the given path  
    let mut res:Vec<String> = Vec::new();
    let directory = match read_dir(path){
        Ok(directory) => directory,
        Err(e) => return res , 
     };

    //item is a DirEntry Struct
    for item in directory{
        let dir = item.unwrap();
        let meta = metadata(dir.path());
        match meta {
            Ok(place) => {
                if place.is_dir() {
                    println!("{}", dir.path().to_str().unwrap());
                    let path_to_dir: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                    let query: path::PathBuf = path::PathBuf::from(&name);
                    if path_to_dir == query {
                        let item:String = String::from(dir.path().to_str().unwrap());
                        res.push(item)
                        }
                    }
                if place.is_file() { 
                    println!("-----{}", dir.path().file_name().unwrap().to_str().unwrap());
                let location: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                let query: path::PathBuf = path::PathBuf::from(&name);
                //println!(" location{:?} query {:?}", location, query);
                if location == query {
                    let item:String = String::from(dir.path().to_str().unwrap());
                    res.push(item)
                }
            
            }else{
                let buff_path = dir.path();
                let str_path = buff_path.to_str().unwrap();
                let string_path:String = String::from(str_path); 
                let new_name = String::from(&name);
                let res2 = recursive_file_search(new_name,string_path );
                    for file in res2 {
                        res.push(file)
                    }
            }
            },
            Err(e) => println!("{:?},{:?} was not sure what to do here with this one",e, dir),
        }
}
res
}
