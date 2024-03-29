use std::{env, usize};


mod flag;
mod print_tree_all;
mod print_tree;
mod print_dirs;
mod print_full_path;
mod print_ignore_pattern;
mod print_prune;
mod print_file_n_limit;
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
            let msg = flag::help(arg_one.clone());
            println!("{}", msg);
            std::process::exit(1); 
       }
       let mut flags:Vec<String> = Vec::new();
       // check to see if a path wnew_dir);iven and if not was it a query?? 
       let mut query:String = String::new();
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
                  let msg = flag::help(String::from(&arg));
                  println!("{}", msg);
              }else if arg.starts_with("-"){
                    let msg = flag::help(String::from(&arg));
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
    let config = match config {
        Ok(config) => config,
        _ =>  Config{
                    path:String::from("/"),
                    query:String::from(""),
                    flag:Vec::new(),
                    } 
    };
    if config.flag.len() > 1 {
        // logic for multiple flags
        if config.flag[0] == String::from("-v") && config.flag[1] == String::from("-f"){
                println!("can not fast search and be verbose, flag combo is not supporeted, remove -f flag");
        } else if config.flag[0] == String::from("-f") && config.flag[1] == String::from("-v"){
                println!("can not fast search and be verbose, flag combo is not supporeted, remove -f flag");
        } else if config.flag[0] == String::from("-v") && config.flag[1] == String::from("-l") {
                        let user_query:String = String::from(&config.query); 
                            println!("{}", &config.path);
                        let res = print_tree::print_tree(config.query,config.path,0);
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
                        let res = print_tree::print_tree(config.query,config.path,0);
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query)
                            }
        } else if config.flag[0] == String::from("-l") && config.flag[1] == String::from("-f") && config.query != String::from(""){

                        let user_query:String = String::from(&config.query); 
                            println!("{}", &config.path);
                        let res = print_tree::print_tree(config.query, config.path,0);      
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query)
                            }
        } else if config.flag[0] == String::from("-f") && config.flag[1] == String::from("-l") && config.query != String::from(""){
                        let user_query:String = String::from(&config.query); 
                            println!("{}", &config.path);
                        let res = print_tree::print_tree(config.query, config.path,0);      
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query)
                            }
        }else{
            println!("did not recognize flag options {}, {}", config.flag[0], config.flag[1]);        
            }
     }else if config.flag.len() == 1 {
        // logic a singal flag passed. Also if statement hell what idiot wrote this... 
         if config.flag[0] == String::from("-a") {
                        let user_query:String = String::from(&config.query); 
                            println!("{} ", &config.path);
                        let res: print_tree_all::Pretty = print_tree_all::recursive_print_all(config.query, config.path,0);      
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query)
                            }
         } else if config.flag[0] == String::from("-d"){ 
                        let user_query:String = String::from(&config.query); 
                            println!("{}", &config.path);
                        let res = print_dirs::print_dirs(config.query, config.path,0);      
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!(" No file or Directory with the name » '{}'", user_query)
                            }
         } else if config.flag[0] == String::from("-f") { 
            let user_query:String = String::from(&config.query); 
            let res = print_full_path::print_full_path(config.query,config.path,0);
                println!(" folders {}, files {} ", res.folders, res.files);
            if res.found.len() > 0 {
                println!("  '{}' was found in the following paths:", user_query);
                    for item in res.found {
                        println!("       {}", item);
                    }
                }else if user_query != ""{
                    println!(" No file or Directory with the name » '{}'", user_query) 
                }
         }  else if config.flag[0] == String::from("-l") { 
            let user_query:String = String::from(&config.query); 
            let res = print_ignore_pattern::print_ignore_pattern(config.query,config.path,0);
                println!(" folders {}, files {} ", res.folders, res.files);
            if res.found.len() > 0 {
                println!("  '{}' was found in the following paths:", user_query);
                    for item in res.found {
                        println!("       {}", item);
                    }
                }else if user_query != "" && config.flag[0] != "-l"{
                    println!(" No file or Directory with the name » '{}'", user_query) 
                }
         }  else if config.flag[0] == String::from("-p") {
                        let user_query:String = String::from(&config.query); 
                            println!("{}", &config.path);
                        let res = print_prune::print_prune(config.query, config.path,0);      
                            println!(" folders {}, files {} ", res.folders, res.files);
                            if res.found.len() > 0 {
                                println!("  '{}' was found in the following paths:", user_query);
                                for item in res.found {
                                    println!("       {}", item);
                                    }
                            }else if user_query != ""{
                                println!("pruned » '{}'", user_query)
                            }else if user_query == "" {
                                println!("...you didn't give me anything to prune...")
                            }
         }  else if config.flag[0] == String::from("--filelimit") || config.flag[0] == String::from("-filelimit") {
                        println!("{}", config.path);
                       let mut n:usize= 0;
                       let parsed: Result<usize, _> = config.query.parse();
                       match parsed {
                        Ok(num)=> {n = num;}
                        Err(e)=> {
                            println!(" You did not specify a limit so I got this error:\n '{}'",e);
                            println!("  `riptree --filelimit 3`");
                       }}
                        let res = print_file_n_limit::print_file_n_limit(config.query, config.path,0,n);
                            println!(" folders {}, files {} ", res.folders, res.files);
         }
     }else
     if config.flag.len() == 0 {
        let user_query:String = String::from(&config.query); 
        let res = print_tree::print_tree(config.query,config.path,0);
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


