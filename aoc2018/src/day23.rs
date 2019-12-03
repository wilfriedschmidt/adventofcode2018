use std::collections::HashMap;
use util::*;

struct Bot
{
  x:i32,
  y:i32,
  z:i32,
  r:i32,
  n:i32,
}

fn dist(bots:&Vec<Bot>, i:usize, j:usize) -> i32
{
  let d = (bots[i].x - bots[j].x).abs() + (bots[i].y - bots[j].y).abs() + (bots[i].z - bots[j].z).abs();
  return d;  
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut bots:Vec<Bot> = Vec::new();

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let parts:Vec<&str> = lines[i].split(',').collect();

      let x = parts[0][5..].parse::<i32>().unwrap();
      let y = parts[1].parse::<i32>().unwrap();
      let z = parts[2][..parts[2].len()-1].parse::<i32>().unwrap();
      let r = parts[3][3..].parse::<i32>().unwrap();
    
      let bot:Bot = Bot { x,y,z,r, n:0};
      bots.push(bot);   
    }
  }

 /* for i in 0..bots.len()
  {
    println!("{} {} {} {}", bots[i].x,bots[i].y,bots[i].z,bots[i].r);
  }*/

  let mut largestr=0;
  let mut largestrindex=0;
  
  for i in 0..bots.len()
  {
    if bots[i].r > largestr
    {
      largestr = bots[i].r;
      largestrindex = i;
    }
  }

  let mut numinrange=0;
  for i in 0..bots.len()
  {
    let d = dist(&bots, i, largestrindex);
    if d <= largestr
    {
      numinrange+=1;
    }
  }
 
  println!("num in range {}",numinrange);

  // get bot with most overlaps
  for i in 0..bots.len()
  {
    for j in i..bots.len()
    {
      let d = dist(&bots, i,j);
      if d <= (bots[i].r+bots[j].r)
      {
        bots[i].n+=1;
      }
    }
  }

  let mut largestn=0;
  let mut largestnindex=0;
  
  for i in 0..bots.len()
  {
    println!("{} num overlapping {}", i, bots[i].n);
    if bots[i].n > largestn
    {
      largestn = bots[i].n;
      largestnindex = i;
    }
  }

  println!("largest count in range {}", largestnindex);

  for j in 0..bots.len()
  {
    if bots[j].n==largestn
    {
      let mut minx = bots[j].x - bots[j].r;
      let mut maxx = bots[j].x + bots[j].r;
      let mut miny = bots[j].y - bots[j].r;
      let mut maxy = bots[j].y + bots[j].r;
      let mut minz = bots[j].z - bots[j].r;
      let mut maxz = bots[j].z + bots[j].r;

      // compute intersection area
      for i in 0..bots.len()
      {
        let d = dist(&bots, largestnindex, i);
        if d <= (bots[j].r+bots[i].r)
        {
          let tminx = bots[i].x - bots[i].r;
          let tmaxx = bots[i].x + bots[i].r;
          let tminy = bots[i].y - bots[i].r;
          let tmaxy = bots[i].y + bots[i].r;
          let tminz = bots[i].z - bots[i].r;
          let tmaxz = bots[i].z + bots[i].r;

          if tminx>minx 
          {
            if tminx < maxx { minx = tminx; }
            else { minx = maxx; }
          }
          if tmaxx<maxx 
          {
            if tmaxx > minx { maxx = tmaxx; }
            else { maxx = minx; }
          }

          if tminy>miny 
          {
            if tminy < maxy { miny = tminy; }
            else { miny = maxy; }
          }
          if tmaxy<maxy
          {
            if tmaxy > miny { maxy = tmaxy; }
            else { maxy = miny; }
          }

          if tminz>minz
          {
            if tminz < maxz { minz = tminz; }
            else { minz = maxz; }
          }
          if tmaxz<maxz
          {
            if tmaxz > minz { maxz = tmaxz; }
            else { maxz = minz; }
          }
        }
      }

      println!("x: {} {} y: {} {} z: {} {}", minx, maxx, miny, maxy, minz, maxz);
    }
  }

  // 37209957 37209957 y: 26012823 38025704 z: 36661066 36661066
  let mut largestyoverlap=0;
  let mut largesty=0;

  let mut count=0;

  let mut miny = 26012823;
  let mut maxy = 38025704;

  let mut y = miny;
  loop  
  {
    count+=1;
    if count%10000==0 { println!("{} of {}, {} {}", count, maxy - miny, largestyoverlap, largesty); }

    let mut overlapcount=0;
    for i in 0..bots.len()
    {
      let d = (bots[i].x - 37209957).abs() + (bots[i].y - y).abs() + (bots[i].z - 36661066).abs();
      if d <= bots[i].r
      {
        overlapcount+=1;
      }
    }

    if overlapcount>largestyoverlap
    {
      largestyoverlap = overlapcount;
      largesty = y;
    }

    y+=1;
    if y>=maxy { break; }
  }

  println!("{} {} {} count {}, dist {}", 37209957, largesty, 36661066, largestyoverlap, 37209957 + largesty + 36661066);
}

