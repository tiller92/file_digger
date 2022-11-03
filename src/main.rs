use file_digger;
use std::env;


// users should pass arg as follows path , query, then flags
fn main() { 
   let args = env::args();
   let config = file_digger::Config::build(args);
   file_digger::handle_args(config)
}
