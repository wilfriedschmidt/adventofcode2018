mod util;
mod day1;

use std::env;

fn main()
{
  let args:Vec<String> = env::args().collect();
  if args[1]=="day1"
  {
    day1::go("../data/day1/test6.txt");
    day1::go("../data/day1/test7.txt");
    day1::go("../data/day1/test8.txt");
    day1::go("../data/day1/input.txt");
  }
}
