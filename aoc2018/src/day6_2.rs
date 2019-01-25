use util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut coords:Vec<Coord> = Vec::new(); 

  let mut minx=100000;
  let mut miny=100000;
  let mut maxx=0;
  let mut maxy=0;

  for i in 0..lines.len()
  {
    if lines[i].len() > 1
    {
      let parts:Vec<&str> = lines[i].split(',').collect();
      let x = parts[0].parse::<i32>().unwrap();
      let y = parts[1][1..].parse::<i32>().unwrap();

      if x < minx
      {
        minx = x;
      }

      if y < miny
      {
        miny = y;
      }

      if x > maxx
      {
        maxx = x;
      }
  
      if y > maxy
      {
        maxy = y;
      }

      let coord:Coord = Coord {x,y};
      coords.push(coord);
    }
  }

  minx-=1;
  miny-=1;

  maxx+=2;
  maxy+=1;

  let gridwidth = maxx-minx;
  let gridheight = maxy-miny;

  println!("{} {} {} {} {} {}", minx, maxx, miny, maxy, gridwidth, gridheight);

  let mut grid:Vec<i32> = Vec::new();
  grid.resize((gridwidth*gridheight) as usize, -1);

  for i in 0..coords.len()
  {
    let x = coords[i].x - minx;
    let y = coords[i].y - miny;

    grid[ (y*gridwidth + x) as usize] = i as i32;
  }

  printgrid(&grid, gridwidth);
  println!("");

  let mut thres:Vec<i32> = Vec::new();
  thres.resize((gridwidth*gridheight) as usize, -1);
   
  for y in 0..gridheight
  {
    for x in 0..gridwidth
    {
      let mut dist = 0;
      for i in 0..coords.len()
      {
        let cx = coords[i].x - minx;
        let cy = coords[i].y - miny;
        dist += (x - cx).abs() + (y - cy).abs();
      }

      if dist < 10000
      {
        thres[ (y*gridwidth + x) as usize] = 1;
      }
    }
  }

  printgrid(&thres, gridwidth);

  let mut count = 0;
  for y in 0..gridheight
  {
    for x in 0..gridwidth
    {
      if thres[ (y*gridwidth + x) as usize] != -1
      {
        count+=1;
      }
    }
  }

  println!("{}", count);
}
