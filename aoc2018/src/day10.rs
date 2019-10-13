use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Point
{
  x:i64,
  y:i64,
  vx:i64,
  vy:i64,
}

impl Default for Point
{
  fn default() -> Point
  {
    Point
    {
      x:0,
      y:0,
      vx:0,
      vy:0
    }
  }
}

pub fn parse(substr:&str) -> i64
{
  let mut temp:String = String::new();
  for i in 0..substr.len()
  {
    let ch = substr.chars().nth(i).unwrap();
    if ch !=' '
    {
      temp.push(ch);
    }
  }

  return temp.parse::<i64>().unwrap();
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut points:Vec<Point> = Vec::new();

  let mut minx = 1000000;
  let mut miny = 1000000;
  let mut maxx = -1000000;
  let mut maxy = -1000000;

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {   
      let x = parse(&lines[i][10..16]);
      let y = parse(&lines[i][18..24]);
      let vx = parse(&lines[i][36..38]);
      let vy = parse(&lines[i][40..42]);
      
      
      if x < minx { minx = x; }
      if y < miny { miny = y; }
      if x > maxx { maxx = x; }
      if y > maxy { maxy = y; }

      points.push( Point {x,y,vx,vy} );
    }
  }

  let mut width = maxx - minx+1;
  let mut height = maxy - miny+1;

  let mut lastwidth = width;
  let mut lastheight = height;

  println!("{} {}", width,  height);

  //let mut display = false;

  for tick in 0..60000
  {
    println!("seconds {}", tick);

    if height == 10
    {
      let mut pointmap:HashMap<i64, bool> = HashMap::new();
      for p in 0..points.len()
      {
        let mut hash:i64 = points[p].x;
        hash = hash << 20;
        hash |= points[p].y;

        pointmap.insert(hash,true);
      }    

      for y in 0..height
      {
        let mut outstr = String::new();
        for x in 0..width
        {
          let cx = x + minx;
          let cy = y + miny;
  
          let mut hash:i64 = cx;
          hash = hash << 20;
          hash |= cy;
          
          let mut found:bool = pointmap.contains_key(&hash);
          if found { outstr.push('#'); }
          else { outstr.push('.'); }
        }
        println!("{}", outstr);

      }

      break;
    }

    minx = 1000000;
    miny = 1000000;
    maxx = -1000000;
    maxy = -1000000;

    for p in 0..points.len()
    {
      points[p].x+=points[p].vx;
      points[p].y+=points[p].vy;

      if points[p].x < minx { minx = points[p].x; }
      if points[p].y < miny { miny = points[p].y; }
      if points[p].x > maxx { maxx = points[p].x; }
      if points[p].y > maxy { maxy = points[p].y; }
    }

    width = maxx-minx+1;
    height = maxy-miny+1;

    println!("width {} height {}", width, height);
  }

}
