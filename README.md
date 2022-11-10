

### File Digger (rip_tree)
 
  # Recreating the popular tree command but with rust. Some goals inlcude:   
        1. implementing popular tree commands,  
        2. creating a verison with similar performance,   
        3. a version with a nice looking command line output,  
        4. no external crate or dependencies,
        5. create a linux friendly version as well
  
   #  Install 
        `cargo install riptree`
   
  # Usage 
        search for a file in a given directory -f flag will not print anything until its done searching.
        ` riptree /path/to/folder 'query' -f `
       

        just calling riptree will display the folders and files of the current directory
        `riptree` (will use your current directory) 

        look for a file fast and skip . files
        `riptree 'query' -f -l`

        search for a file or directory in the current directory 
        `riptree 'query'`
        
        for more riptree --help
        `riptree --help`
