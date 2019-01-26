use range::*;
use coord::*;
use grid::*;

pub fn loaddata(lines:&Vec<&str>, coords:&mut Vec<Coord>, xrange:&mut Range, yrange:&mut Range, grid:&mut Grid<i32>)
{
  for i in 0..lines.len()
  {
    if lines[i].len() > 1
    {
      let parts:Vec<&str> = lines[i].split(',').collect();
      let x = parts[0].parse::<i32>().unwrap();
      let y = parts[1][1..].parse::<i32>().unwrap();

      if x < xrange.min
      {
        xrange.min = x;
      }

      if y < yrange.min
      {
        yrange.min = y;
      }

      if x > xrange.max
      {
        xrange.max = x;
      }
  
      if y > yrange.max
      {
        yrange.max = y;
      }

      let coord:Coord = Coord {x,y};
      coords.push(coord);
    }
  }

  xrange.min-=1;
  yrange.min-=1;

  xrange.max+=2;
  yrange.max+=1;

  grid.width = xrange.max-xrange.min;
  grid.height = yrange.max-yrange.min;

  println!("{} {} {} {} {} {}", xrange.min, xrange.max, yrange.min, yrange.max, grid.width, grid.height);

  grid.data.resize((grid.width*grid.height) as usize, -1);

  for i in 0..coords.len()
  {
    let x = coords[i].x - xrange.min;
    let y = coords[i].y - yrange.min;

    grid.put(x,y,i as i32);
  }
}
