use std::collections::HashMap;
use util::*;

struct Claim
{
  id:i32,

  x:i32,
  y:i32,

  width:i32,
  height:i32,
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut claims:Vec<Claim> = Vec::new();

  for i in 0..lines.len()
  {
    if lines[i].len() > 1
    {
      let parts:Vec<&str> = lines[i].split(' ').collect();

      let id = parts[0][1..].parse::<i32>().unwrap();
      
      let coords:Vec<&str> = parts[2].split(',').collect();
      let dim:Vec<&str> = parts[3].split('x').collect();
     
      let x = coords[0].parse::<i32>().unwrap();
      let y = coords[1][..coords[1].len()-1].parse::<i32>().unwrap();

      let width = dim[0].parse::<i32>().unwrap();
      let height = dim[1].parse::<i32>().unwrap();

      let c:Claim = Claim { id, x,y, width, height };

      claims.push(c);
    }
  }

  for i in 0..claims.len()
  {
    println!("id:{} x:{} y:{} width:{} height:{}", claims[i].id, claims[i].x, claims[i].y, claims[i].width, claims[i].height); 
  }

  let fabricwidth = 1000 as usize;

  let mut fabric:Vec<i32> = Vec::new();
  fabric.resize(fabricwidth*fabricwidth,0);

  let mut mymap = HashMap::new();

  for i in 0..claims.len()
  {
    for y in 0..claims[i].height
    {
      for x in 0..claims[i].width
      {
        let nx = x + claims[i].x;
        let ny = y + claims[i].y;

        let offset = (ny*(fabricwidth as i32)+nx) as usize;
        if fabric[offset]==0
        {
          fabric[offset] = claims[i].id;
        }
        else
        {
          if !mymap.contains_key(&fabric[offset])
          {
            mymap.insert(fabric[offset],1);
          }
          if !mymap.contains_key(&claims[i].id)
          {
            mymap.insert(claims[i].id,1);
          }

          fabric[offset] = -1;
        }
      } 
    }
  }

  let mut count:i32=0;
  for i in 0..(fabricwidth*fabricwidth)
  {
    if fabric[i]==-1
    {
      count+=1;
    }
  }

  println!("count {}", count);

  for i in 0..claims.len()
  {
    if !mymap.contains_key(&claims[i].id)
    {
      println!("not overlapped {}", claims[i].id);
    }
  }

}
