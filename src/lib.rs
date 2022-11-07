use std::env;
mod flag;
mod recurse;
mod recursive_file_search;
// add a flag option 
pub struct Config {
    pub path:String,
    pub query:String, 
}


impl Config {
    // handle all command line interface in build pass a to recursice func type config
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        args.next();
       // get the local path in case no path is given 
       let local_dir = match env::current_dir(){
            Ok(l_path) => l_path,
            Err(_e) => return Err("problem getting current directory {e}" ),
       };
       let mut user_path:String = String::from(local_dir.to_str().unwrap());
       let string_path_id = "/";
       let path = match args.next(){
            Some(path)=>path,
            None => String::from(local_dir.to_str().unwrap()),
       };
       
       let mut query:String = String::from(" "); 
       if path.starts_with("--help"){
            let msg = flag::flags(path.clone());
            println!("{}", msg);
            return Ok(Config{
                path:String::from("help"),
                query:String::from(" "),
            });
        }
       if path.contains(string_path_id){
            user_path = path.clone();
       }else if path.len() > 0{
          query = path;
       }
       
       // check for a string that doesnt start with a '-' that will be out query
       let args_after_two:Vec<String> = args.collect();
          for arg in args_after_two {
              if arg.starts_with("--help"){
                  let msg = flag::flags(String::from(&arg));
                  println!("{}", msg);
              }else if arg.contains("/"){
                  println!("another path?")
                      
              }else if arg.starts_with("-"){
                    let msg = flag::flags(String::from(&arg));
                    println!("{}",msg);
              }else {
                 query = String::from(&arg);
              }
       }  
       Ok(Config{
           path:user_path,
           query:query,
       })
    }
}


// handle arg length and finalize funcionality
pub fn run(config:Result<Config, &'static str>){ 
    //fn really just handles the config struct and handles errors before we start recursing 
    let config = match config {
        Ok(config) => config,
        _ =>  Config{
                    path:String::from("/"),
                    query:String::from(""),
                    } 
    };
    let user_query:String = String::from(&config.query); 
    let res = recursive_file_search::recursive_file_search(config.query,config.path);
    println!(" folders {}, files {} ", res.folders, res.files);
    if res.found.len() > 0 {
    println!("  '{}' was found in the following paths:", user_query);
        for item in res.found {
                println!("       {}", item);
        }
    }else if user_query != " "{
        println!(" No file or Directory with the name » '{}'", user_query) 
    }
}


