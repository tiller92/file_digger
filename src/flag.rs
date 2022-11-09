
pub fn flags(flag:String)-> String {
    let mut _msg:String = String::from(" ");
    if flag == "--help" {
         _msg = String::from("
            
            riptree will search a folders for a file you pass it. It also displays the folders and files 
            in the given path.
            
            example:
                riptree folder_name/folder_name 'file_name.txt'

                
            ");
    }
    _msg
}
