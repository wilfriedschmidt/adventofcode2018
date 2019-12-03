use std::collections::HashMap;
use util::*;

pub fn go(serialnum:i64)
{

  let mut grid:Vec<i64> = Vec::new();
  grid.resize(300*300,0);

  for uy in 0..300
  {
    for ux in 0..300
    {
      let x:i64 = ux+1 as i64;
      let y:i64 = uy+1 as i64;

      let mut power:i64 = (x+10)*y + serialnum;
      power = power * (x+10);
      power = power / 100;
      power = power % 10;
      power = power - 5;

      grid[(uy * 300 + ux) as usize] = power;
    }
  }

  let mut sums:Vec<i64> = Vec::new();
  sums.resize(300*300*300,0);

  for y in 0..300
  {
    println!("y {}", y);
    for size in 1..300
    {
      for x in 0..(300-size)
      {
        let mut totalsum = 0;
        if size>1
        {
          totalsum = sums[ (size-2)*300*300 + (y*300+x)];
        }
        totalsum+=grid[ (y * 300 + x+(size-1)) as usize];

        sums[ (size-1)*300*300 + (y*300+x)] = totalsum;
      }
    }
  }

  let mut greatestsum=0;
  let mut greatestx=0;
  let mut greatesty=0;
  let mut greatestsize=0;

  for size in 1..300
  {
    println!("size {}", size);
    for x in 0..(300-size)
    {
      for y in 0..(300-size)
      {
        let mut totalsum=0;
        for s in 0..size
        {
          totalsum+=sums[ (size-1)*300*300 + ((y+s)*300+x)];
        }

        if totalsum > greatestsum
        {
          greatestsum=totalsum;
          greatestx=x+1;
          greatesty=y+1;
          greatestsize=size;
        }        
      }
    }
  }

  println!("{} {} {}", greatestx, greatesty, greatestsize);

}
