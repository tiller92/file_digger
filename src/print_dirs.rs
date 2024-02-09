// -a or -all this should show or search all dir even hidden files this should
// be tht pretty version so breath first display 
use std::fs::{metadata,read_dir};
use std::path;


pub struct Pretty {
    pub found: Vec<String>,
    pub folders: u32,
    pub files:u32,
}

pub fn print_dirs(name: String, path:String, depth:u32) -> Pretty{
  let mut file_count:u32 = 0;
  let mut folder_count:u32 = 0;
  let mut res:Vec<String> = Vec::new();
  let directory = match read_dir(path.clone()){
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
  // queue of folders that need printed while there is a queue of fodler
  for item in directory{ 
    // each current folder and file 
      let dir = item.unwrap();
      let meta = metadata(dir.path());
      match meta {
          Ok(place) => {
            if dir.file_name().to_str().unwrap().starts_with(".") == false{
                  let mut dash_string = String::from("  ");
                  let mut dash_file = String::from("  ");
                  for _i in 0..depth{
                    dash_string.push('|');
                    dash_string.push(' ');
                    dash_string.push(' ');
                    dash_file.push('|');
                    dash_file.push(' ');
                    dash_file.push(' ');
                    
                  } 
              if place.is_dir() {
            // print folder
            // now print each folder and file
                  folder_count = folder_count + 1;
                  println!("{}|-{}",dash_string, dir.file_name().to_str().unwrap());
                  let path_to_dir: path::PathBuf = path::PathBuf::from(dir.path().file_name().unwrap());
                  let query: path::PathBuf = path::PathBuf::from(&name);
                  if path_to_dir == query {
                      let item:String = String::from(dir.path().to_str().unwrap());
                      res.push(item)
                      }
                  }

                    //if its not a file then it pass that path to the function again
                  let buff_path = dir.path();
                  let str_path = buff_path.to_str().unwrap();
                  let string_path:String = String::from(str_path); 
                  let new_name = String::from(&name);
                  let res2 = print_dirs(new_name,string_path, depth+1);
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

