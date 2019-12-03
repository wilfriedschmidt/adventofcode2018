use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Constellation
{
  x:i32,
  y:i32,
  z:i32,
  w:i32,
  prev:Vec<i32>,
  next:Vec<i32>,
  visited:bool,
}

fn dist(left:&Constellation, right:&Constellation) -> i32
{
  let mut d = (left.x - right.x).abs();
  d += (left.y - right.y).abs();
  d += (left.z - right.z).abs();
  d += (left.w - right.w).abs();

  return d;
}

fn find_lowest_index(cons:&mut Vec<Constellation>, index:usize) -> i32
{
  if cons[index].visited { return index as i32; }
  cons[index].visited = true;

let mut lowest = index;

  for i in 0..cons[index].prev.len()
  {
    if cons[index].prev[i] != index as i32
    {
      let prev = cons[index].prev[i] as usize;
      let res = find_lowest_index(cons, prev);
      if res < (lowest as i32) { lowest = res as usize; }
    }
  }

  for i in 0..cons[index].next.len()
  {
    if cons[index].next[i] != index as i32
    {
      let next = cons[index].next[i] as usize;
      let res = find_lowest_index(cons, next);
      if res < (lowest as i32) { lowest = res as usize; }
    }
  }

  return lowest as i32;
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut constellations:Vec<Constellation> = Vec::new();

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let parts:Vec<&str> = lines[i].split(',').collect();
      let x=parts[0].parse::<i32>().unwrap();
      let y=parts[1].parse::<i32>().unwrap();
      let z=parts[2].parse::<i32>().unwrap();
      let w=parts[3].parse::<i32>().unwrap();
      let con:Constellation = Constellation { x,y,z,w, prev:Vec::new(), next:Vec::new(), visited:false };
      constellations.push(con);
    }
  }

  // build links
  for i in 0..constellations.len()
  {
    for j in i+1..constellations.len()
    {
      let d = dist(&constellations[i], &constellations[j]);
      if d<=3
      {
        constellations[j].prev.push(i as i32);
        constellations[i].next.push(j as i32);
      }
    }
  }

  let mut roots:HashMap<i32,bool> = HashMap::new();

  // traverse links
  for i in 0..constellations.len()
  {
    for j in 0..constellations.len()
    {
      constellations[j].visited=false;
    }

    let lowest = find_lowest_index(&mut constellations, i);
    println!("lowest index for {} is {}", i, lowest);
    if !roots.contains_key(&lowest) { roots.insert(lowest,true); } 
  }

  for i in 0..constellations.len()
  {
    println!("{} {} {} {}  {} {} ", constellations[i].x, constellations[i].y, constellations[i].z, constellations[i].w, i, constellations[i].visited); 
    
    let mut outstr = String::new();
    for j in 0..constellations[i].prev.len()
    {
      outstr.push_str(&constellations[i].prev[j].to_string());  
      outstr.push(' ');    
    }
    println!("  prev: {}", outstr);

    outstr = String::new();
    for j in 0..constellations[i].next.len()
    {
      outstr.push_str(&constellations[i].next[j].to_string());  
      outstr.push(' ');    
    }
    println!("  next: {}", outstr);
  }

  println!("\n");

  println!("num roots {}", roots.len());
}
