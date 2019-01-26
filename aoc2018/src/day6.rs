use std::collections::HashMap;
use util::*;
use range::*;
use coord::*;
use grid::*;
use day6_util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut coords:Vec<Coord> = Vec::new();
  let mut xrange:Range = Range::default();
  let mut yrange:Range = Range::default();
  let mut grid:Grid<i32> = Grid::default();
 
  loaddata( &lines, &mut coords, &mut xrange, &mut yrange, &mut grid );

  grid.print();
  println!("");
    
  for i in 0..coords.len()
  {
    let cx = coords[i].x - xrange.min;
    let cy = coords[i].y - yrange.min;

    for y in 0..grid.height
    {
      for x in 0..grid.width
      {
        let dist = (x - cx).abs() + (y - cy).abs();

        let mut minindex = i as i32;
        for j in 0..coords.len()
        {
          if i != j
          {
            let cx2 = coords[j].x - xrange.min;
            let cy2 = coords[j].y - yrange.min;
            let dist2 = (x - cx2).abs() + (y - cy2).abs();

            if dist2<=dist
            {
              minindex = -1;
              break;
            }
          }  
        }

        if minindex != -1
        {
          grid.put(x,y,minindex as i32);
        }
      }
    }
  }

  grid.print();

  let mut infinite = HashMap::new();
  let mut area = HashMap::new();

  for y in 0..grid.height
  {
    for x in 0..grid.width
    {
      let index = grid.get(x,y);

      if *index != -1
      {
        if area.contains_key(&index)
        {
          *area.get_mut(&index).unwrap() += 1;
        }
        else
        {
          area.insert(index,1);
        }      

        if (x==0) || (x==grid.width-1) || (y==0) || (y==grid.height-1)
        {
          if !infinite.contains_key(&index)
          {
            infinite.insert(index,1);
          }
        }
      }
    }
  }

  for (key,value) in area
  {
    if !infinite.contains_key(&key)
    {
      println!("{} {}", key, value);
    }
  }
}
