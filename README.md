

## File Digger (rip_tree)
 
  # Recreating the popular tree command but with rust. Some goals inlcude:   
        1. implementing popular tree commands,  
        2. creating a verison with similar performance,   
        3. a version with a nice looking command line output,  
        4. no external crates or dependencies,
        5. search functionality that returns a list of paths 
            where a file or directory was found
  
#     Install 
        `cargo install riptree`
   
#     Usage (more functionality coming) 

        just calling riptree will display the folders and files of the current directory
        `riptree` (will use your current directory will not look through or display . folders or) 

        pass riptree a path to start somewhere besides the current directory
        `riptree /some/path/to/somewhere/`

        display dot folder and files 
        `riptree -a`

        search for a file or directory in the current directory 
        `riptree 'query'`
        
        for more riptree --help
        `riptree --help`
