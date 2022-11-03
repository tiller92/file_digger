
pub fn flags(flag:String)-> String {
    let mut msg:String = String::from(" ");
    if flag == "--help" {
         msg = String::from("
            riptree will search a folder for a string you pass it. It also displays the folders and files 
                    in the given path.
            ");
    }
    msg
}
