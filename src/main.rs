use file_digger;


fn main() { 
    // this should call a function that takes a string and searches through the file system
   let name: String = String::from("help");
   file_digger::search_for_name(name);
}
