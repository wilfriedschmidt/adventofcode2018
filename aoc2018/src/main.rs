mod util;
mod grid;
mod coord;
mod range;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6_util;
mod day6;
mod day6_2;
mod day7;
mod day7_2;
mod day7_util;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day12_2;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day22_2;
mod day22_3;
mod day23;
mod day24;
mod day25;

use std::env;

fn main()
{
  let args:Vec<String> = env::args().collect();

  if args.len()<=1
  {
    println!("enter day");
  } 
  else
  { 
    if args[1]=="day1" || args[1]=="all"
    {
      //day1::go("../data/day1/test6.txt");
      //day1::go("../data/day1/test7.txt");
      //day1::go("../data/day1/test8.txt");
      day1::go("../data/day1/input.txt");
    }

    if args[1]=="day2" || args[1]=="all"
    {
      //day2::go("../data/day2/test1.txt");
      //day2::go("../data/day2/test2.txt");
      day2::go("../data/day2/input.txt");
    }

    if args[1]=="day3" || args[1]=="all"
    {
      //day3::go("../data/day3/test1.txt");
      day3::go("../data/day3/input.txt");
    }

    if args[1]=="day4" || args[1]=="all"
    {
      //day4::go("../data/day4/test1.txt");
      day4::go("../data/day4/input.txt");
    }

    if args[1]=="day5" || args[1]=="all"
    {
      //day5::go("../data/day5/test1.txt");
      day5::go("../data/day5/input.txt");
    }

    if args[1]=="day6" || args[1]=="all"
    {
      //day6::go("../data/day6/test1.txt");
      day6::go("../data/day6/input.txt");
    }

    if args[1]=="day6_2" || args[1]=="all"
    {
      //day6_2::go("../data/day6/test1.txt");
      day6_2::go("../data/day6/input.txt");
    }

    if args[1]=="day7" || args[1]=="all"
    {
      //day7::go("../data/day7/test1.txt");
      day7::go("../data/day7/input.txt");
    }

    if args[1]=="day7_2" || args[1]=="all"
    {
      //day7_2::go("../data/day7/test1.txt");
      day7_2::go("../data/day7/input.txt");
    }

    if args[1]=="day8" || args[1]=="all"
    {
      //day8::go("../data/day8/test1.txt");
      day8::go("../data/day8/input.txt");
    }

    if args[1]=="day9" || args[1]=="all"
    {
      //day9::go(9,25);
      //day9::go(30,5807);
      //day9::go(425,70848);
      day9::go(425,7084800);
    }

    if args[1]=="day10" || args[1]=="all"
    {
      day10::go("../data/day10/input.txt");
    }

    if args[1]=="day11" || args[1]=="all"
    {
      //day11::go(18);
      //day11::go(42);
      day11::go(6042);
    }

    if args[1]=="day12" || args[1]=="all"
    {
      //day12::go("../data/day12/test1.txt","#..#.#..##......###...###");  
      day12::go("../data/day12/input.txt","##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#");
    }

    if args[1]=="day12_2" || args[1]=="all"
    {
      //day12_2::go("../data/day12/test1.txt","#..#.#..##......###...###");
      day12_2::go("../data/day12/input.txt","##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#");
    }

    if args[1]=="day13" || args[1]=="all"
    {
      //day13::go("../data/day13/test1.txt");
      //day13::go("../data/day13/test2.txt");
      day13::go("../data/day13/input.txt");
    }

    if args[1]=="day14" || args[1]=="all"
    {
      day14::go(633601);
      //day14::go(59414);
    }

    if args[1]=="day15" || args[1]=="all"
    {
      day15::go("../data/day15/input.txt");
      //day15::go("../data/day15/test_27730.txt");
      //day15::go("../data/day15/test_18740.txt");
      //day15::go("../data/day15/test_27755.txt");
      //day15::go("../data/day15/test_28944.txt");
      //day15::go("../data/day15/test_36334.txt");
      //day15::go("../data/day15/test_39514.txt");
      //day15::go("../data/day15/test_1140.txt");
      //day15::go("../data/day15/test_6474.txt");
    }

    if args[1]=="day16" || args[1]=="all"
    {
      //day17::go("../data/day17/test1.txt");
      day16::go("../data/day16/input.txt");
    }

    if args[1]=="day17" || args[1]=="all"
    {
      //day17::go("../data/day17/test1.txt");
      day17::go("../data/day17/input.txt");
    }

    if args[1]=="day18" || args[1]=="all"
    {
      //day18::go("../data/day18/test1.txt");
      day18::go("../data/day18/input.txt");
    }

    if args[1]=="day19" || args[1]=="all"
    {
      //day19::go("../data/day19/test1.txt");
      day19::go("../data/day19/input.txt");
    }

    if args[1]=="day20" || args[1]=="all"
    {
      //day20::go("../data/day20/test1.txt");
      day20::go("../data/day20/input.txt");
    }

    if args[1]=="day21" || args[1]=="all"
    {
      //day21::go("../data/day21/test1.txt");
      day21::go("../data/day21/input.txt");
    }

    if args[1]=="day22" || args[1]=="all"
    {
      day22_2::go(7305,13,734);
      //day22_3::go(510,10,10);
      //day22_3::go(7305,13,734);
      //day22_3::go(6969,9,796);
    }

    if args[1]=="day23" || args[1]=="all"
    {
      //day23::go("../data/day23/test1.txt");
      day23::go("../data/day23/input.txt");
    }

    if args[1]=="day24" || args[1]=="all"
    {
      //day24::go("../data/day24/test1.txt");
      day24::go("../data/day24/input.txt");
    }

    if args[1]=="day25" || args[1]=="all"
    {
      //day25::go("../data/day25/test1.txt");
      day25::go("../data/day25/input.txt");
    }
  }
}
