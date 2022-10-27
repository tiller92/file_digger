use file_digger;


fn main() { 
    // this should call a function that takes a string and searches through the file system
   let name: String = String::from("help");
   let path: String = String::from("/Users/ryantiller/Documents");
   file_digger::search_for_name(name,path);
}
