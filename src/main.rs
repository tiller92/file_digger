use riptree;
use std::env;


fn main() { 
   let args = env::args();
   let config = riptree::Config::build(args);
   riptree::handle_args(config)
}
