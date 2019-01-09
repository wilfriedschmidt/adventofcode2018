mod util;
mod day1;
mod day2;
mod day3;
mod day4;

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

  if args[1]=="day2"
  {
    day2::go("../data/day2/test1.txt");
    day2::go("../data/day2/test2.txt");
    day2::go("../data/day2/input.txt");
  }

  if args[1]=="day3"
  {
    day3::go("../data/day3/test1.txt");
    day3::go("../data/day3/input.txt");
  }

  if args[1]=="day4"
  {
    day4::go("../data/day4/test1.txt");
    day4::go("../data/day4/input.txt");
  }
}
