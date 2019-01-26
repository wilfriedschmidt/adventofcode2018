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

  let mut thres:Grid<i32> = Grid::default();
  thres.width = grid.width;
  thres.height = grid.height;
  thres.data.resize((thres.width*thres.height) as usize, -1);
   
  for y in 0..grid.height
  {
    for x in 0..grid.width
    {
      let mut dist = 0;
      for i in 0..coords.len()
      {
        let cx = coords[i].x - xrange.min;
        let cy = coords[i].y - yrange.min;
        dist += (x - cx).abs() + (y - cy).abs();
      }

      if dist < 10000
      {
        thres.put(x,y,1);
      }
    }
  }

  let mut count = 0;
  for y in 0..grid.height
  {
    for x in 0..grid.width
    {
      if *thres.get(x,y) != -1
      {
        count+=1;
      }
    }
  }

  println!("{}", count);
}
