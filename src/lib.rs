use std::fs::{metadata,read_dir};
use std::path;

#[derive(Debug)]
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

pub fn handle_args(config:Result<Config,&'static str>){ 
    //fn really just handles the config struct and handles errors before we start recursing 
   let config = match config {
        Ok(config) => config,
        _ => Config{path:String::from("/"), query:String::from(" ") }, 
   }; 
    recursive_file_search(config.query,config.path);
}

pub fn recursive_file_search(name: String, path:String ) {
   //Direcotory is an iterator that contains all the files or folders on the given path  
    let directory = match read_dir(path){
        Ok(directory) => directory,
        Err(e) => return println!("{:?} make sure the path argument leads to a directory", e),
     };

    //item is a DirEntry Struct
    for item in directory{
        let dir = item.unwrap();
        let meta = metadata(dir.path());
        match meta {
            Ok(place) => {
                if place.is_dir() {println!("{}", dir.path().to_str().unwrap());};
                if place.is_file() { 
                    println!("-----{}", dir.path().file_name().unwrap().to_str().unwrap());
                let location: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                let query: path::PathBuf = path::PathBuf::from(&name);
                //println!(" location{:?} query {:?}", location, query);
                if location == query {
                  //  println!("found location");
                  }
            
            }else{
                let buff_path = dir.path();
                let str_path = buff_path.to_str().unwrap();
                let string_path:String = String::from(str_path); 
                let new_name = String::from(&name);
                recursive_file_search(new_name,string_path )
                }
            },
            Err(e) => println!("{:?},{:?} was not sure what to do here with this one",e, dir),
        }

}
}
