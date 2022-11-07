use riptree;
use std::env;


fn main() { 
   let args = env::args();
   let config = riptree::Config::build(args);
   riptree::run(config)
}
