//
//  Advent of Code Challenge, Solution for Day 15:
//  https://adventofcode.com/2018/day/15
//
//
//  MIT License
//
//  Copyright (c) 2019 Wilfried Schmidt
//
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.
//

use std::collections::HashMap;
use util::*;
use coord::*;
use grid::*;

#[derive(Clone)]
struct Entity
{
  what:char,
  coord:Coord,
  hp:i32,
}

#[derive(Clone)]
struct Node
{
  coord:Coord,
  myindex:usize,
  parent:usize,
  g:i32,
  h:i32,
}
  
fn coord_order(coord:&Coord, width:i32) -> i32
{
  return coord.y*width+coord.x;
}

fn write_entities_into_grid(grid:&mut Grid<char>, entities:&Vec<Entity>)
{
  for i in 0..entities.len()
  {
    if entities[i].hp > 0
    {
      grid.put(entities[i].coord.x, entities[i].coord.y, entities[i].what);
    }  
  }
}

fn print_entities(entities:&Vec<Entity>)
{
  for i in 0..entities.len()
  {
    println!("{} x:{} y:{} hp:{}", entities[i].what, entities[i].coord.x, entities[i].coord.y, entities[i].hp);
  }
}

fn dist(x:i32, y:i32, dest:&Coord) -> i32
{
  return (dest.x - x).abs() + (dest.y - y).abs();
}

fn try_path_to(grid:&Grid<char>, source:&Coord, dest:&Coord) -> Vec<Coord>
{

let mut allnodes:Vec<Node> = Vec::new();
let mut open:Vec<Node> = Vec::new();
let mut path:Vec<Coord> = Vec::new();
let mut tempgrid = grid.clone();

  let root:Node = Node { coord:source.clone(), myindex:0, parent:0, g:0, h:dist(source.x, source.y,&dest) }; 
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
    // pick the one with the lowest reading order

    let xd:[i32;4] = [0,-1,1,0];
    let yd:[i32;4] = [-1,0,0,1];
    for i in 0..4
    {
      let x = next.coord.x + xd[i];
      let y = next.coord.y + yd[i];

      if *tempgrid.get(x,y) == '.'
      {
        // block it off
        tempgrid.put( x, y, 'X');

        let tempcoord:Coord = Coord {x,y};
        let toadd:Node = Node { coord:tempcoord, myindex:allnodes.len(), parent:next.myindex, g: (next.g+1), h:dist(x,y,&dest) };
        allnodes.push(toadd.clone());
        open.push(toadd.clone());

        if x==dest.x && y==dest.y
        {
          // path was found return it
          let mut currentindex = toadd.myindex;          
          loop
          {
            path.push(Coord {x:allnodes[currentindex].coord.x, y:allnodes[currentindex].coord.y} );                        
            currentindex = allnodes[currentindex].parent;
            if currentindex==0
            {
              path.reverse();
              return path;
            }
          }
        }
      }
    }
  }

  return path;
}

fn write_path_into_grid(grid:&mut Grid<char>, path:&Vec<Coord>)
{
  for i in 0..path.len()
  {
    grid.put(path[i].x, path[i].y, 'X');
  }
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();
 
  let mut grid:Grid<char> = Grid::default();
 
  grid.height = (lines.len()-1) as i32;
  grid.width = (lines[0].len()) as i32;
  grid.data.resize((grid.width*grid.height) as usize,' ');

  println!("width {} height {}", grid.width, grid.height);

  let mut entities:Vec<Entity> = Vec::new();
  let mut backupentities:Vec<Entity> = Vec::new();

  for y in 0..lines.len()
  {
    if lines[y].len()>1
    { 
      for x in 0..lines[y].len()
      {
        let ch = lines[y].chars().nth(x).unwrap();

        let xi = x as i32;
        let yi = y as i32;

        let coord:Coord = Coord {x:xi,y:yi};
        if ch=='E' || ch=='G'
        {
          let entity:Entity = Entity { what:ch, coord, hp:200 };
          entities.push(entity);

          grid.put( x as i32, y as i32, '.');
        }
        else
        { 
          grid.put( x as i32, y as i32, ch);
        }
      }
    }
  }

  backupentities = entities.clone();
  
  let mut temp:Grid<char> = grid.clone();
  write_entities_into_grid(&mut temp, &entities);
  
  temp.print();
  print_entities(&entities);


  let xd:[i32;4] = [0,-1,1,0];
  let yd:[i32;4] = [-1,0,0,1];

  let mut round=0;
  let mut numelves=0;
  let mut numgoblins=0;

  let mut elfhp = 0;
  let mut goblinhp = 0;

  let mut minattack=20;

  let mut startover=false;

  loop
  {
    
    round=0;
    startover = false;
    entities = backupentities.clone();

    loop
    {
      // sort entities
      entities.sort_by_key(|c| coord_order(&c.coord, grid.width));        

      for i in 0..entities.len()
      {

        temp = grid.clone();
        write_entities_into_grid(&mut temp, &entities);

        if entities[i].hp > 0
        {
          let mut enemy = ' ';
          if entities[i].what=='E' { enemy = 'G'; }
          if entities[i].what=='G' { enemy = 'E'; }

          // find target to move to
          let mut mindist = 10000000;
          let mut minpath:Vec<Coord> = Vec::new();
          let mut mindest:Coord = Coord{x:0,y:0};

          for j in 0..entities.len()
          {
            if entities[j].what==enemy && entities[j].hp>0
            {
              for k in 0..4
              {
                let ex = entities[j].coord.x + xd[k];
                let ey = entities[j].coord.y + yd[k];

                if ex==entities[i].coord.x && ey==entities[i].coord.y
                {
                  // already at target
                  mindist = 0;
                  minpath = Vec::new();
                }
                else
                {
                  let source = Coord {x:entities[i].coord.x, y:entities[i].coord.y};
                  let dest = Coord {x:ex, y:ey};

                  let path = try_path_to(&temp,&source,&dest);
                  if path.len()>0 && path.len() < mindist
                  {
                    mindist = path.len();
                    minpath = path;
                    mindest = dest.clone();
                  }
                }
              }            
            }
          }

          if minpath.len()>0
          {
            // do move
            let mut coordset = false;
            for k in 0..4
            {
              let sx = entities[i].coord.x + xd[k];
              let sy = entities[i].coord.y + yd[k];

              if *temp.get(sx,sy)=='.'
              {
                let source = Coord {x:sx, y:sy};
                let path = try_path_to(&temp,&source,&mindest);
                if path.len()>0 && path.len() == (mindist-1)
                {
                  entities[i].coord = source.clone();
                  coordset = true;
                  break;
                }
              }
            }

            if !coordset
            {
              entities[i].coord = minpath[0].clone();
            }
            
            // re-raster          
            temp = grid.clone();
            write_entities_into_grid(&mut temp, &entities);

          }

          // find attack target
          let mut minhp = 100000000;
          let mut mintarget=-1;
          for k in 0..4
          {
            let ex = entities[i].coord.x + xd[k];
            let ey = entities[i].coord.y + yd[k];
          
            for j in 0..entities.len()
            {
              if entities[j].what==enemy && entities[j].hp>0
              {
                if entities[j].coord.x==ex && entities[j].coord.y==ey
                {
                  if entities[j].hp < minhp
                  {
                    minhp = entities[j].hp;
                    mintarget = j as i32;
                  }
                  break;
                }
              }
            }
          }

          if mintarget>=0
          {
            if enemy=='E'
            {
              entities[mintarget as usize].hp-=3;

              if entities[mintarget as usize].hp<=0
              {
                minattack+=1;
                startover=true;
                println!("starting over {}", minattack);
              }
            }
            else
            {
              entities[mintarget as usize].hp-=minattack;
            }

            // re-raster          
            temp = grid.clone();
            write_entities_into_grid(&mut temp, &entities);
          }

        }
      }

      if startover {break; }
      
      round+=1;

      entities.sort_by_key(|c| coord_order(&c.coord, grid.width));        


      println!("\nround {}", round);
      let mut temp:Grid<char> = grid.clone();
      write_entities_into_grid(&mut temp, &entities);
      temp.print();
      print_entities(&entities);


      numelves=0;
      numgoblins=0;

      elfhp = 0;
      goblinhp = 0;

      for i in 0..entities.len()
      {
        if entities[i].hp > 0
        {
          if entities[i].what=='G' { numgoblins+=1; goblinhp+=entities[i].hp; }
          if entities[i].what=='E' { numelves+=1; elfhp+=entities[i].hp; }
        }
      }

      // combat is over
      if numelves==0 || numgoblins==0 { break; }

    }

    if numgoblins==0 { break; }
  }

  println!("final round {}", round);

  if numelves>0 {println!("elfs {} {}", elfhp, (round-1)*elfhp); }
  if numgoblins>0 {println!("goblins {} {}", goblinhp, (round-1)*goblinhp); }

  println!("final attack {}", minattack);
}
