use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Grid
{
  data:Vec<i64>,
  width:i32,
  height:i32,
}

impl Default for Grid
{
  fn default() -> Grid
  {
    Grid
    {
      data:Vec::new(),
      width:0,
      height:0,
    }
  }
}
 
fn put_grid(grid:&mut Grid, x:i32, y:i32, value:i64)
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    grid.data[ (y*grid.width + x) as usize ] = value;
  }
}

fn get_grid(grid:&Grid, x:i32, y:i32) -> i64
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    return grid.data[ (y*grid.width + x) as usize ].clone();
  }
  return -1;
}

fn print_grid(grid:&Grid)
{
  for y in 0..grid.height
  {
    let mut outstr = String::new();
    for x in 0..grid.width
    {
      let val = get_grid(grid,x,y);
      if val==0 { outstr.push('.'); }
      if val==1 { outstr.push('='); }
      if val==2 { outstr.push('|'); }
      if val==3 { outstr.push('#'); }
      if val>=4 { outstr.push_str(&(val-4).to_string()); }
    }
    println!("{}",outstr);
  } 
}

#[derive(Clone,PartialEq,Eq,Hash)]
struct Coord
{
  x:i32,
  y:i32,
  cost:i32,
}

#[derive(Clone)]
struct Node
{
  coord:Coord,
  equiped:i32,
  myindex:usize,
  parent:usize,
  g:i32,
  h:i32,
}

fn dist(x:i32, y:i32, dest:&Coord) -> i32
{
  return (dest.x - x).abs() + (dest.y - y).abs();
}

// 0 rocky
// 1 wet
// 2 narrow

// 0 climbing gear
// 1 torch
// 2 neither

fn cost(equiped:i32, from:i64, to:i64, hints:&[i32;3]) -> (i32,i32) 
{
  if from==to { return (1,equiped); }

  if from==0 // from rocky, climbing or torch equiped
  {
    if to==1 // to wet
    {
      if equiped==0 { return (1,equiped); } // can use climbing gear
      else  // otherwise switch to best hint, climbing or neither
      {
        if hints[0] > hints[2] { return (7,0); }
        else { return (7,2); }
      }
    }
    else if to==2 // to narrow
    {
      if equiped==1 { return (1,equiped); } // can use torch
      else  // otherwise switch to best hint, torch or neither
      {
        if hints[1] > hints[2] { return (7,1); }
        else { return (7,2); }
      }
    }
    else { println!(" error to is {} ", to); }
  }
  else if from==1 // from wet, climbing or neither
  {
    if to==0 // to rocky
    {
      if equiped==0 { return (1,equiped); } // can use climbing
      else  // otherwise switch to best hint, climbing or torch
      {
        if hints[0] > hints[1] { return (7,0); }
        else { return (7,1); }
      }
    }
    else if to==2 // to narrow
    {
      if equiped==2 { return (1,equiped); } // can use neither
      else  // otherwise switch to best hint, torch or neither
      {
        if hints[1] > hints[2] { return (7,1); }
        else { return (7,2); }
      }
    }
    else { println!(" error to is {} ", to); }
  }
  else if from==2 // from narrow, torch or neither
  {
    if to==0 // to rocky
    {
      if equiped==1 { return (1,equiped); } // can use torch
      else  // otherwise switch to best hint, climbing or torch
      {
        if hints[0] > hints[1] { return (7,0); }
        else { return (7,1); }
      }
    }
    else if to==1 // to wet
    {
      if equiped==2 { return (1,equiped); } // can use neither
      else  // otherwise switch to best hint, climbing or neither
      {
        if hints[0] > hints[2] { return (7,0); }
        else { return (7,2); }
      }
    }
    else { println!(" error to is {}", to); }
  }
  else { println!("error from is {}", from); }

  return (0,0);
}

fn try_path_to(grid:&Grid, source:&Coord, dest:&Coord) -> (Vec<Coord>,i32)
{

let mut allnodes:Vec<Node> = Vec::new();
let mut open:Vec<Node> = Vec::new();
let mut path:Vec<Coord> = Vec::new();
let mut tempgrid = grid.clone();

  let root:Node = Node { coord:source.clone(), equiped: 0, myindex:0, parent:0, g:0, h:dist(source.x, source.y,&dest) }; 
  allnodes.push(root.clone());
  open.push(root.clone());

  loop
  {
    open.sort_by_key(|n| n.g+n.h);
    open.reverse();

    if open.len()==0
    {
      break;
    }

    let next:Node = open.pop().unwrap();

    let from = get_grid(&grid, next.coord.x, next.coord.y);
    
    let xd:[i32;4] = [0,-1,1,0];
    let yd:[i32;4] = [-1,0,0,1];
    for i in 0..4
    {
      let x = next.coord.x + xd[i];
      let y = next.coord.y + yd[i];

      let to = get_grid(&tempgrid, x, y);

      // get best equipped option from this position, should be closest match to here

      if to != 3
      {
        // this is a candidate node to expand to
        let mut hints:[i32;3] = [0,0,0];

        for j in 0..4
        {
          let sx = x + xd[i];
          let sy = y + yd[i];

          // 0 rocky climbing or torch equiped
          // 1 wet climbing or neither
          // 2 narrow torch or neither

          // 0 climbing gear
          // 1 torch
          // 2 neither

          let subto = get_grid(&tempgrid, sx, sy);
          if subto==0
          {
            hints[0]+=1;
            hints[1]+=1;
          }
          if subto==1
          {
            hints[0]+=1;
            hints[2]+=1;
          }
          if subto==2
          {
            hints[1]+=1;
            hints[2]+=1;
          }
        }

        // tie break with where I am at
        /*if to==0
        {
          hints[0]+=1;
          hints[1]+=1;
        }
        if to==1
        {
          hints[0]+=1;
          hints[2]+=1;
        }
        if to==2
        {
          hints[1]+=1;
          hints[2]+=1;
        }*/

        // block it off
        put_grid(&mut tempgrid, x, y, 3);

        let res = cost(next.equiped, from, to, &hints);
        let newcost = res.0;
        let newequiped = res.1;

        let tempcoord:Coord = Coord {x,y, cost:newcost};
        let toadd:Node = Node { coord:tempcoord, equiped:newequiped, myindex:allnodes.len(), parent:next.myindex, g: (next.g+newcost), h:dist(x,y,&dest) };
        allnodes.push(toadd.clone());
        open.push(toadd.clone());

        if x==dest.x && y==dest.y
        {
          // path was found return it
          let mut currentindex = toadd.myindex;          
          loop
          {
            path.push(Coord {x:allnodes[currentindex].coord.x, y:allnodes[currentindex].coord.y, cost:allnodes[currentindex].coord.cost} );                        
            currentindex = allnodes[currentindex].parent;
            if currentindex==0
            {
              path.reverse();
              return (path, allnodes[toadd.myindex].g);
            }
          }
        }
      }
    }
  }

  return (path,0);
}

fn write_path_into_grid(grid:&mut Grid, path:&Vec<Coord>)
{
  for i in 0..path.len()
  {
    put_grid(grid, path[i].x, path[i].y, (4+path[i].cost) as i64);
  }
}

pub fn go(depth:i64, tx:i32, ty:i32)
{

/*
  let mut a:BigNum = BigNum::default();
  let mut b:BigNum = BigNum::default();
  let mut c:BigNum = BigNum::default();
  
  a.words[0] = 255;
  b.words[0] = 7;
  c = mul_bignum(&a,&b);

  println!("{} {} {} {}", c.words[0].to_string(), c.words[1].to_string(), c.words[2].to_string(), c.words[3].to_string());
*/

  let width = tx+1000;
  let height = ty+1000;

  let mut geoindex:Grid = Grid::default();
  geoindex.width = width;
  geoindex.height = height;
  geoindex.data.resize((geoindex.width*geoindex.height) as usize, 0);

  let mut erosion:Grid = Grid::default();
  erosion.width = width;
  erosion.height = height;
  erosion.data.resize((erosion.width*erosion.height) as usize, 0);

  let mut typegrid:Grid = Grid::default();
  typegrid.width = width+1;
  typegrid.height = height+1;
  typegrid.data.resize((typegrid.width*typegrid.height) as usize, 3);

  for x in 1..geoindex.width
  {
    let res = (x*16807) as i64;
    put_grid(&mut geoindex, x, 0, res );
    put_grid(&mut erosion, x, 0, (res + depth) % 20183);
  }
  for y in 1..geoindex.height
  {
    let res = (y*48271) as i64;
    put_grid(&mut geoindex, 0, y, res );
    put_grid(&mut erosion, 0, y, (res + depth) % 20183);
  }

  for y in 1..geoindex.height
  {
    for x in 1..geoindex.width
    {
      let top = get_grid(&erosion, x, y-1);
      let left = get_grid(&erosion, x-1, y);

      let res = top*left;

      put_grid(&mut geoindex, x, y, res);
      put_grid(&mut erosion, x, y, (res + depth) % 20183);
    }
  }
  
  put_grid(&mut erosion, tx, ty, 0);

  let mut risk = 0;
  for y in 0..(ty+1)
  {
    for x in 0..(tx+1)
    {
      risk += get_grid(&erosion,x,y) % 3;
    }
  }

  risk -= get_grid(&erosion,tx,ty) % 3;

  println!("risk {}", risk);

  for y in 0..geoindex.height
  {
    for x in 0..geoindex.width
    {
      let gtype = get_grid(&erosion, x, y) % 3;
      put_grid(&mut typegrid, x+1, y+1, gtype);
    }
  }

  //print_grid(&typegrid);

  let source = Coord {x:1, y:1, cost:0};
  let dest = Coord {x:tx+1, y:ty+1, cost:0};

  let res = try_path_to(&typegrid, &dest, &source);
  let path = res.0;

  write_path_into_grid(&mut typegrid, &path);
  print_grid(&typegrid);

  println!("total cost {}", res.1);
}
