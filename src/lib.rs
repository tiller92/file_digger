use std::env;
mod flag;
mod recurse;
mod recursive_file_search;
mod no_dot_files_verbose_path;


pub struct Config {
    pub path:String,
    pub query:String, 
    pub flag:Vec<String>,
}


impl Config {
    // handle all command line interface in build pass a to recursice func type config
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        args.next();
       // gets the local directory path in case the user does not give a path
        let local_dir = match env::current_dir(){
            Ok(l_path) => l_path,
            Err(_e) => return Err("problem getting current directory {e}" ),
       };
       let mut user_path:String = String::from(local_dir.to_str().unwrap());
       // next arg is either a path or a flag could be nothing if nothing return local path
       let arg_one = match args.next(){
            Some(path)=>path,
            None => String::from(local_dir.to_str().unwrap()),
       };
       
       if arg_one.starts_with("--help"){
            // ref &str is better to pass here
            let msg = flag::flags(arg_one.clone());
            println!("{}", msg);
            std::process::exit(1); 
       }
       // check to see if a path was given and if not was it a query?? 
       let mut query:String = String::new();
       let mut flags:Vec<String> = Vec::new();
       //string_path_id should me more thourogh. Most likely a fn that returns a bool.
       let string_path_id = "/";
           if arg_one.contains(string_path_id){
                user_path = arg_one.clone();
           }else if arg_one.len() > 0 && arg_one.starts_with("-") == false {
                query = arg_one.clone();
           }else {
                flags.push(arg_one);
           }
       
       // check for a string that doesnt start with a '-' that will be out query
       let args_after_two:Vec<String> = args.collect();
          for arg in args_after_two {
              if arg.starts_with("--help"){
                  let msg = flag::flags(String::from(&arg));
                  println!("{}", msg);
                      
              }else if arg.starts_with("-"){
                    let msg = flag::flags(String::from(&arg));
                    println!("{}",msg);
                    flags.push(arg)
              }else {
                 query = String::from(&arg);
              }
       } 
       Ok(Config{
           path:user_path,
           query:query,
           flag:flags,
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
                    flag:Vec::new(),
                    } 
    };
       // set func variable and that gets filled with the func needed to satisfy res
    if config.flag.len() > 1 {
        // logic for multiple flags
        if config.flag[0] == String::from("-v") && config.flag[1] == String::from("-f"){
                println!("can not fast search and be verbose, flag combo is not supporeted, remove -f flag");
        } else if config.flag[0] == String::from("-f") && config.flag[1] == String::from("-v"){
                println!("can not fast search and be verbose, flag combo is not supporeted, remove -f flag");
        } else if config.flag[0] == String::from("-v") && config.flag[1] == String::from("-l") {
                        let user_query:String = String::from(&config.query); 
                        println!("{}", &config.path);
                        let res = no_dot_files_verbose_path::no_dot_files_verbose_path(config.query,config.path);
                            println!(" folders {}, files {} ", res.folders, res.files);
                        if res.found.len() > 0 {
                            println!("  '{}' was found in the following paths:", user_query);
                            for item in res.found {
                                println!("       {}", item);
                                }
                        }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query) 
                                }
        } else if config.flag[0] == String::from("-l") && config.flag[1] == String::from("-v") {
                        let user_query:String = String::from(&config.query); 
                            println!("{}", &config.path);
                        let res = no_dot_files_verbose_path::no_dot_files_verbose_path(config.query,config.path);
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query)
                            }
        } else if config.flag[0] == String::from("-l") && config.flag[1] == String::from("-f"){
                            println!("fast search and skip . files");
        } else if config.flag[0] == String::from("-f") && config.flag[1] == String::from("-l"){
                            println!("fast search and skip . files");
        }else{
            println!("did not recognize flag options {}, {}", config.flag[0], config.flag[1]);        
            }
     }else 
     if config.flag.len() == 1 {
         if config.flag[0] == String::from("-l") {
            println!("pretty recurse but ignore . files");
             } else if config.flag[0] == String::from("-f") && config.query != String::from(""){
                 println!("fast search needs a query ")
                 } else if config.flag[0] == String::from("-v") { 
                     println!("be verbose with file path");
                 }
     }else
     if config.flag.len() == 0 {
        let user_query:String = String::from(&config.query); 
        let res = recursive_file_search::recursive_file_search(config.query,config.path);
            println!(" folders {}, files {} ", res.folders, res.files);
        if res.found.len() > 0 {
            println!("  '{}' was found in the following paths:", user_query);
            for item in res.found {
                println!("       {}", item);
            }
        }else if user_query != ""{
            println!(" No file or Directory with the name » '{}'", user_query) 
        }
     }
                 
}


