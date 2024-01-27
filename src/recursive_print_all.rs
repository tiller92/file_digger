// -a or -all this should show or search all dir even hidden files 
use std::fs::{metadata,read_dir};
use std::path;


pub struct Pretty {
    pub found: Vec<String>,
    pub folders: u32,
    pub files:u32,
}

pub fn recursive_print_all(name: String, path:String ) -> Pretty{
  // Slow hopefully cool looking verions 
  // need to add counts for files and folders
  let mut file_count:u32 = 0;
  let mut folder_count:u32 = 0;
  

  let mut res:Vec<String> = Vec::new();
  let directory = match read_dir(path){
      Ok(directory) => directory,
      Err(_) =>  {
      let fail = Pretty {
                  found:res,
                  folders:0,
                  files:0,
              };   
      return fail    
      } , 
   };
   // this loops though the currents path dirs and prints it folders and files but I need it print all of the folder and files of the
   // current directory first.
  for item in directory{
      let dir = item.unwrap();
      let meta = metadata(dir.path());
      match meta {
          Ok(place) => {
              if place.is_dir() && folder_count > 0{
                  folder_count = folder_count +1;
                  println!(" {}", dir.path().to_str().unwrap());
                  let path_to_dir: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                  let query: path::PathBuf = path::PathBuf::from(&name);
                  if path_to_dir == query {
                      let item:String = String::from(dir.path().to_str().unwrap());
                      res.push(item)
                      }
                  }else if place.is_dir() && folder_count == 0 {
                    // this is to catch the original path and print it files
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
                  println!("   »» {}", dir.path().file_name().unwrap().to_str().unwrap());
                  let location: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                  let query: path::PathBuf = path::PathBuf::from(&name);
                      if location == query {
                          let item:String = String::from(dir.path().to_str().unwrap());
                          res.push(item)
                        }
                }else{
                    //if its not a file then it pass that path to the function again
                  let buff_path = dir.path();
                  let str_path = buff_path.to_str().unwrap();
                  let string_path:String = String::from(str_path); 
                  let new_name = String::from(&name);
                  let res2 = recursive_print_all(new_name,string_path );
                      for file in res2.found {
                          res.push(file)
                      }

                  folder_count = folder_count + res2.folders;
                  file_count = file_count + res2.files;
                  }
          },
          Err(e) => println!("{},{} was not sure what to do here with this one" ,e, dir.path().to_str().unwrap())
      }
  }
  let res_pretty = Pretty {
      found: res,
      folders: folder_count,
      files:file_count,
  };
  res_pretty
}



