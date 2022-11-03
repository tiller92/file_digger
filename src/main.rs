use riptree;
use std::env;


// users should pass arg as follows path , query, then flags
fn main() { 
   let args = env::args();
   let config = riptree::Config::build(args);
   riptree::handle_args(config)
}
