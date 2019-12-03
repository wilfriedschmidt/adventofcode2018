use std::collections::HashMap;
use util::*;
use grid::*;

#[derive(Clone)]
struct Vein
{
  xmin:usize,
  xmax:usize,
  
  ymin:usize,
  ymax:usize,
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut veins:Vec<Vein> = Vec::new();

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let sparts:Vec<&str> = lines[i].split(' ').collect();

      let mut v:Vein = Vein {xmin:0, xmax:0, ymin:0, ymax:0};


      // x=568, y=1564..1578
      if lines[i].chars().nth(0).unwrap()=='x'
      {
        v.xmin = sparts[0][2..sparts[0].len()-1].parse::<usize>().unwrap();
        v.xmax = v.xmin;

        let yparts:Vec<&str> = sparts[1].split('.').collect();

        //println!("{} {}", yparts[0], yparts[2]);
        v.ymin = yparts[0][2..yparts[0].len()].parse::<usize>().unwrap();
        v.ymax = yparts[2][0..].parse::<usize>().unwrap();
        
      }
      else 
      {
        v.ymin = sparts[0][2..sparts[0].len()-1].parse::<usize>().unwrap();
        v.ymax = v.ymin;

        let xparts:Vec<&str> = sparts[1].split('.').collect();
        v.xmin = xparts[0][2..xparts[0].len()].parse::<usize>().unwrap();
        v.xmax = xparts[2][0..].parse::<usize>().unwrap();
      }
        
      veins.push(v);
    }
  }

  let mut minx=10000000;
  let mut miny=10000000;  

  let mut maxx=0;
  let mut maxy=0;  

  let mut width=0;
  let mut height=0;

  for i in 0..veins.len()
  {
    if minx>veins[i].xmin { minx=veins[i].xmin; }
    if maxx<veins[i].xmax { maxx=veins[i].xmax; }

    if miny>veins[i].ymin { miny=veins[i].ymin; }
    if maxy<veins[i].ymax { maxy=veins[i].ymax; }

    println!("{} {} {} {}", veins[i].xmin, veins[i].xmax, veins[i].ymin, veins[i].ymax);
  }

  let ominy=miny;
  let omaxy=maxy;

  minx-=2;
  miny=0;

  width = (maxx - minx)+4;
  height = (maxy - miny)+2;

  println!("{} {} {} {} {} {}", width, height, minx, maxx, miny, maxy);

  let mut grandtotal=0;

  let mut grid:Grid<char> = Grid::default();
  grid.width = width as i32;
  grid.height = height as i32;
  grid.data.resize(width*height,'.');

  let mut processed:Grid<char> = Grid::default();
  processed.width = width as i32;
  processed.height = height as i32;
  processed.data.resize(width*height,' ');

  for i in 0..veins.len()
  {
    if veins[i].xmax==veins[i].xmin
    {
      for j in veins[i].ymin..veins[i].ymax+1
      {
        let x = (veins[i].xmin - minx) as i32;
        let y = (j - miny) as i32;

        grid.put(x,y,'#');
      }
    }
    else
    {
      for j in veins[i].xmin..veins[i].xmax+1
      {
        let y = (veins[i].ymin - miny) as i32;
        let x = (j - minx) as i32;

        grid.put(x,y,'#');
     }
    }
  }

  grid.put((500-minx) as i32, (0-miny) as i32, '+');

  loop
  {
    let mut wateradded=false;
  
    // expand drops
    for y in 0..grid.height
    {
      for x in 0..grid.width
      {
        let ch = *grid.get(x,y);
        let p = *processed.get(x,y);
        if ch=='|' || ch=='+'
        {
          let below = *grid.get(x,y+1);
          if below=='.'
          {
            // gravity
            wateradded = true;
            grid.put(x, y+1, '|');
          }
          else if (below=='#' || below=='~') && p==' ' 
          {
            let mut leftpool = -1;
            let mut rightpool = -1;

            let mut tx = x-1;
            // go left
            loop
            {
              let here = *grid.get(tx,y);
              let below = *grid.get(tx,y+1);
              if below=='|' { leftpool = tx+1; break; }
              if below=='.' { leftpool = tx; break; }
              if here == '#' { leftpool = tx+1; break; }
              tx-=1;
              if tx<0 { break; }
            }

            tx = x+1;
            // go right
            loop
            {
              let here = *grid.get(tx,y);
              let below = *grid.get(tx,y+1);
              if below=='|' { rightpool = tx-1; break; }
              if below=='.' { rightpool = tx; break; }
              if here == '#' { rightpool = tx-1; break; }
              tx+=1;
              if tx>=grid.width { break; }
            }

            if leftpool>=0 && rightpool>=0
            {
              println!("expanding {} {}..{}", y, leftpool, rightpool);
              for i in leftpool..rightpool+1
              {
                if *grid.get(i,y) != '|'
                {
                  wateradded = true;
                  grid.put(i,y,'|');
                  processed.put(i,y, '+');
                }
              }
            }
          }
        }
      }
    }

    // convert to pools
    for y in 0..grid.height
    {
      for x in 0..grid.width
      {        
        let ch = *grid.get(x,y);
        let p = *processed.get(x,y);
        if ch=='|' && p==' '
        {
          //println!("scanning {} {}", x,y);
          let mut leftpool = -1;
          let mut rightpool = -1;

          let mut tx = x-1;
          // go left
          loop
          {
            let here = *grid.get(tx,y);
            if here == '#' { leftpool = tx+1; break; }
            if here != '|' { break; }
            tx-=1;
            if tx<0 { break; }
          }

          tx = x+1;
          // go right
          loop
          {
            let here = *grid.get(tx,y);            
            if here == '#' { rightpool = tx-1; break; }
            if here != '|' { break; }
            tx+=1;
            if tx>=grid.width { break; }
          }

          if leftpool>=0 && rightpool>=0
          {
            println!("filling {} {}..{}", y, leftpool, rightpool);
            for i in leftpool..rightpool+1
            {
              if *grid.get(i,y) == '|'
              {
                wateradded = true;
                grid.put(i,y,'~');                
                processed.put(i,y, '+');
              }
            }
          }
        }
      }
    }
   
    println!(""); 

    if !wateradded {break;}

  }

 
  // count
  let mut total=0;
  let mut notdrained=0;
  for y in ominy..omaxy+1
  {
    for x in 0..grid.width
    {
      let ch=*grid.get(x,(y-miny) as i32);
      if ch=='~' || ch=='|' { total+=1; }
      if ch=='~' { notdrained+=1; }
    }
  }

  grid.print();

  println!("total {} not drained {} ",total, notdrained);
}
