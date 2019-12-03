use std::collections::HashMap;
use util::*;
 
#[derive(Clone)]
struct GridNode
{
  terrain:i8, // 0 rock, 1 wet, 2 narrow, 3 impassable
  equiped:i8, // 0 climbing, 1 torch, 2 neither, 3 na
  cost:i32,
  shortcost:i32,
  previndex:usize,
  prevdir:i32,
}

impl Default for GridNode
{
  fn default() -> GridNode
  {
    GridNode
    {
      terrain:3,
      equiped:3,
      cost:100000,
      shortcost:100000,
      previndex:0,
      prevdir:4,
    }
  }
}

#[derive(Clone)]
struct GridOffset
{
  x:i64,
  y:i64,
  z:i64,
  cost:i32,
}

#[derive(Clone)]
struct Grid
{
  data:Vec<i64>,
  width:i64,
  height:i64,
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
 
fn put_grid(grid:&mut Grid, x:i64, y:i64, value:i64)
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    grid.data[ (y*grid.width + x) as usize ] = value;
  }
}

fn get_grid(grid:&Grid, x:i64, y:i64) -> i64
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
      if val==3 { outstr.push('X'); }
    }
    println!("{}",outstr);
  } 
}

fn offset(width:i64, x:i64, y:i64) -> i64
{
  return y*width+x;
}

pub fn go(depth:i64, tx:i64, ty:i64)
{

  /*
  let mut expand=1000;
  let mut lowestequiped1 = 10000;
  loop
  {

    expand+=1;

    println!("\nexpand {}", expand);
*/
    // generate terrain
    let width = tx*20;
    let height = ty*20;

    let mut geoindex:Grid = Grid::default();
    geoindex.width = width;
    geoindex.height = height;
    geoindex.data.resize((geoindex.width*geoindex.height) as usize, 0);

    let mut erosion:Grid = Grid::default();
    erosion.width = width;
    erosion.height = height;
    erosion.data.resize((erosion.width*erosion.height) as usize, 0);

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
        if x==tx && y==ty
        {
          put_grid(&mut geoindex, x, y, 0);
          put_grid(&mut erosion, x, y, depth % 20183);
        }
        else
        {
          let top = get_grid(&erosion, x, y-1);
          let left = get_grid(&erosion, x-1, y);

          let res = top*left;

          put_grid(&mut geoindex, x, y, res);
          put_grid(&mut erosion, x, y, (res + depth) % 20183);
        }
      }
    }
    
    //put_grid(&mut erosion, tx, ty, 0);

    let mut risk = 0;
    for y in 0..(ty+1)
    {
      for x in 0..(tx+1)
      {
        risk += get_grid(&erosion,x,y) % 3;
      }
    }

    risk -= get_grid(&erosion,tx,ty) % 3;

    let mut basegrid:Grid = erosion.clone();
    for i in 0..basegrid.data.len() { basegrid.data[i] %= 3; } 

    //print_grid(&basegrid);
    //println!("risk {}", risk);

    // dijkstra the map
    let twidth = width+1;
    let theight = height+1;
    let mut typegrid:[Vec<GridNode>;3] = [Vec::new(),Vec::new(),Vec::new()];
    let gridsize = (twidth*theight) as usize;
    typegrid[0].resize( gridsize, GridNode::default());
    typegrid[1].resize( gridsize, GridNode::default());
    typegrid[2].resize( gridsize, GridNode::default());

    for y in 0..geoindex.height
    {
      for x in 0..geoindex.width
      {
        let gtype = get_grid(&erosion, x, y) % 3;
        let toffset = offset(twidth, x+1, y+1) as usize;

        // rocky or wet
        if gtype==0 || gtype==1 { typegrid[0][ toffset ].terrain = gtype as i8; }
        else { typegrid[0][ toffset ].terrain = 3; }

        typegrid[0][ toffset ].equiped = 0;

        // rocky or narrow
        if gtype==0 || gtype==2 { typegrid[1][ toffset ].terrain = gtype as i8; }
        else { typegrid[1][ toffset ].terrain = 3; }

        typegrid[1][ toffset ].equiped = 1;

        // wet or narrow
        if gtype==1 || gtype==2 { typegrid[2][ toffset ].terrain = gtype as i8; }
        else { typegrid[2][ toffset ].terrain = 3; }

        typegrid[2][ toffset ].equiped = 2;
      }
    }

    let startoffset = offset(twidth,1,1) as usize; // start at torch
    typegrid[1][startoffset].cost = 0;
    typegrid[1][startoffset].shortcost = 0;

    let mut unvisited:Vec<GridOffset> = Vec::new();
    unvisited.push(GridOffset { x:1, y:1, z:1, cost:0 });

    loop
    {
      unvisited.sort_by_key(|x| x.cost);
      unvisited.reverse();
      let next = unvisited.pop().unwrap();

      //println!("\nprocessing {} {} {} {}", next.x, next.y, next.z, next.cost); 

      let x = next.x;
      let y = next.y;
      let z = next.z as usize;
      let myoffset = offset(twidth,x,y) as usize;
      let myequiped = typegrid[z][myoffset].equiped;
      let mycost = typegrid[z][myoffset].cost;

      /*if mycost != next.cost
      {
        println!("error costs don't match");
      }*/

      if typegrid[z][myoffset].terrain==3
      {
        println!("wrong terrain in unvisited");
      }

      if myequiped==3
      {
        println!("uninit equiped");
      }

      // change equiped
      for i in 0..2
      {
        let nz = ((z + i + 1)%3) as usize;

        let noffset = offset(twidth, x, y) as usize;
        let nterrain = typegrid[nz][noffset].terrain;
        let ncost = typegrid[nz][noffset].cost;

        //println!("checking {} {} {}  {} {} {}", x,y,nz, mycost, myequiped, nterrain);

        let mut newcost = mycost;
        let mut newshortcost = 0;
        if nterrain != 3 
        {
          newcost+=7;
          newshortcost = 7;
        }

        //println!("  newcost {}", newcost);

        if newcost != mycost && newcost < ncost
        {
          if typegrid[nz][noffset].cost == 100000
          {
            unvisited.push( GridOffset { x, y, z:nz as i64, cost:newcost } );
          }

          typegrid[nz][noffset].cost = newcost;
          typegrid[nz][noffset].shortcost = newshortcost;
          typegrid[nz][noffset].previndex = myoffset;
          typegrid[nz][noffset].prevdir = i as i32;
        }
      }


      // move along plane
                      //d, r, l, u
      let xd:[i64;4] = [0,-1,1,0];
      let yd:[i64;4] = [-1,0,0,1];
      for i in 0..4
      {
        let nx = x + xd[i];
        let ny = y + yd[i];

        if nx < twidth && ny < theight
        {
          let noffset = offset(twidth, nx,ny) as usize;
          let nterrain = typegrid[z][noffset].terrain;
          let ncost = typegrid[z][noffset].cost;

          //println!("checking {} {} {}  {} {} {}", nx,ny,z, mycost, myequiped, nterrain);

          let mut newcost = mycost;
          let mut newshortcost = 0;     
          if nterrain==0 // moving to rocky, need climbing or torch
          {
            if myequiped==0 || myequiped==1
            {
              newcost+=1;
              newshortcost = 1;
            }
          }
          else if nterrain==1 // moving to wet, need climbing or neither
          {
            if myequiped==0 || myequiped==2
            {
              newcost+=1;
              newshortcost = 1;
            }
          }
          else if nterrain==2 // moving to narrow, need torch or neither
          {
            if myequiped==1 || myequiped==2
            {
              newcost+=1;
              newshortcost = 1;
            }
          }
          
          //println!("  newcost {}", newcost);

          if newcost != mycost && newcost < ncost
          {
            if typegrid[z][noffset].cost == 100000
            {
              unvisited.push( GridOffset { x:nx, y:ny, z:z as i64, cost:newcost } );
            }

            typegrid[z][noffset].cost = newcost;
            typegrid[z][noffset].shortcost = newshortcost;
            typegrid[z][noffset].previndex = myoffset;
            typegrid[z][noffset].prevdir = i as i32;
          }
        }
      } // for

      if unvisited.len()==0 { break; }
    }

  /*
    for y in 0..theight
    {
      let mut outstr1 = String::new();
      let mut outstr2 = String::new();

      for x in 0..twidth
      {
        let offset = (y*twidth+x) as usize;

        let terrain = typegrid[offset].terrain;
        if terrain==0 { outstr1.push('.'); } 
        else if terrain==1 { outstr1.push('='); }
        else if terrain==2 { outstr1.push('|'); }
        else if terrain==3 { outstr1.push('#'); }

        let equiped1 = typegrid[offset].equiped1;
        if equiped1==0 { outstr1.push('c'); } 
        else if equiped1==1 { outstr1.push('t'); }
        else if equiped1==2 { outstr1.push('n'); }
        else if equiped1==3 { outstr1.push('_'); }

        let equiped2 = typegrid[offset].equiped2;
        if equiped2==0 { outstr1.push('c'); } 
        else if equiped2==1 { outstr1.push('t'); }
        else if equiped2==2 { outstr1.push('n'); }
        else if equiped2==3 { outstr1.push('_'); }

        outstr1.push(' ');

        if typegrid[offset].shortcost < 10 { outstr2.push_str(&typegrid[offset].shortcost.to_string()); }
        else { outstr2.push('_'); }

        let dir = typegrid[offset].prevdir;
        if dir==0 { outstr2.push('d'); }
        else if dir==1 { outstr2.push('r'); }
        else if dir==2 { outstr2.push('l'); }
        else if dir==3 { outstr2.push('u'); }
        else { outstr2.push('_'); }

        outstr2.push_str("  ");
      }

      println!("{}", outstr1);
      println!("{}\n", outstr2);
    }

    let mut x = tx+1;
    let mut y = ty+1;

    let mut totalcost = 0;
    loop
    {
      let offset = (y*twidth + x) as usize;
      let dir = typegrid[offset].prevdir;
      totalcost += typegrid[offset].shortcost;

      let bgridoffset = ((y-1)*width + (x-1)) as usize;
      basegrid.data[bgridoffset] = 3;

      if dir==0 { y+=1; }
      if dir==1 { x+=1; }
      if dir==2 { x-=1; }
      if dir==3 { y-=1; }
      
      if x==1 && y==1 { break; }
    } 

    print_grid(&basegrid);

    println!("totalcost {}", totalcost);*/

    for i in 0..3
    {
      let targetoffset = offset(twidth, tx+1, ty+1) as usize;

 //     if i==1 && typegrid[ targetoffset ].cost < lowestequiped1
   //   {
     //   lowestequiped1 = typegrid[ targetoffset ].cost;
      //}

      println!("cost at target {} equiped {} type {}", typegrid[i][ targetoffset ].cost, typegrid[i][ targetoffset ].equiped, typegrid[i][ targetoffset ].terrain);
    }

    //println!("lowest with torch {}", lowestequiped1);
  //}
}
