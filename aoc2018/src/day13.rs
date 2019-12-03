//
//  Advent of Code Challenge, Solution for Day 13:
//  https://adventofcode.com/2018/day/13
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

#[derive(Clone)]
struct Cart
{
  x:usize,
  y:usize,
  dir:char,
  state:usize,
  collided:bool,
}

impl Default for Cart
{
  fn default() -> Cart
  {
    Cart
    {
      x:0,
      y:0,
      dir:' ',
      state:0,
      collided:false,
    }
  }
}

fn put_char(grid:&mut Vec<char>, width:usize, height:usize, x:usize, y:usize, value:char)
{
  if x>=0 && x<width && y>=0 && y<height
  {
    grid[y*width+x] = value;
  }
}

fn get_char(grid:&Vec<char>, width:usize, height:usize, x:usize, y:usize) -> char
{
  if x>=0 && x<width && y>=0 && y<height
  {
    return grid[y*width+x];
  }
  return ' ';
}

fn print_grid(grid:&Vec<char>, width:usize, height:usize)
{
  for i in 0..height
  {
    let mut outstr = String::new();
    for j in 0..width
    {
      //outstr.push(grid[(i*width+j) as usize]);
      outstr.push(get_char(grid,width,height,j,i));
    }
    println!("|{}|",outstr);
  } 
}

fn print_grid_with_carts(grid:&Vec<char>, carts:&Vec<Cart>, width:usize, height:usize)
{
  for i in 0..height
  {
    let mut outstr = String::new();
    for j in 0..width
    {
      let mut wrote=false;
      for k in 0..carts.len()
      {
        if carts[k].x==j && carts[k].y==i
        {
          if carts[k].collided { outstr.push('X'); }
          else { outstr.push(carts[k].dir); }
          wrote=true;
          break;
        }
      }
        
      if !wrote { outstr.push(get_char(grid,width,height,j,i)); }
    }
    println!("|{}|",outstr);
  } 
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let height = lines.len()-1;
  let width = lines[0].len();

  println!("width {} height {}", width, height);

  let mut grid:Vec<char> = Vec::new();
  grid.resize(width*height,' ');

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      
      for j in 0..lines[i].len()
      {
        put_char(&mut grid, width,height, j,i, lines[i].chars().nth(j).unwrap());
      }
    }
  }

  // grab all carts
  // replace with correct, connections
  let mut carts:Vec<Cart> = Vec::new();
  for y in 0..height
  {
    for x in 0..width
    {
      let ch = get_char(&grid,width,height,x,y);
      if ch=='<' || ch=='>' || ch=='^' || ch=='v'
      {
        let mut cart = Cart {x,y,dir:ch, state:0, collided:false};
        carts.push(cart);

        let mut top=' ';
        let mut left=' ';
        let mut right=' ';
        let mut bottom=' ';

        if y > 0 { top = get_char(&grid,width,height,x,y-1); }
        if x > 0 { left = get_char(&grid,width,height,x-1,y); }
        right = get_char(&grid,width,height,x+1,y);
        bottom = get_char(&grid,width,height,x,y+1);
        
        let mut put=false;

        if top=='|' && bottom=='|' && left=='-' && right=='-' { put_char(&mut grid, width, height, x,y, '+'); put=true; }
        if !put && top=='|' && bottom=='|' { put_char(&mut grid, width, height, x,y, '|'); put=true; }
        if !put && left=='-' && right=='-' { put_char(&mut grid, width, height, x,y, '-'); put=true; }
        if !put && top=='|' && left=='-'  { put_char(&mut grid, width, height, x,y, '/'); put=true; }
        if !put && top=='|' && right=='-'  { put_char(&mut grid, width, height, x,y, '\\'); put=true; }
        if !put && bottom=='|' && left=='-'  { put_char(&mut grid, width, height, x,y, '\\'); put=true; }
        if !put && bottom=='|' && right=='-'  { put_char(&mut grid, width, height, x,y, '/'); put=true; }

        if !put && (bottom=='+' || bottom=='|') && (top=='\\' || top=='/') { put_char(&mut grid, width, height, x,y, '|'); put=true; }
        if !put && (top=='+' ||  top=='|')  && (bottom=='\\' || bottom=='/') { put_char(&mut grid, width, height, x,y, '|'); put=true; }

        if !put && (left=='+' || left=='-') && (top=='\\' || top=='/') { put_char(&mut grid, width, height, x,y, '-'); put=true; }
        if !put && (right=='+' || right=='-') && (top=='\\' || top=='/') { put_char(&mut grid, width, height, x,y, '-'); put=true; }

        if !put && (top=='+' || top=='|') && (bottom=='+' || bottom=='|') { put_char(&mut grid, width, height, x,y, '|'); put=true; }
        if !put && (left=='+' || left=='-') && (right=='+' || right=='-') { put_char(&mut grid, width, height, x,y, '-'); put=true; }

        if !put && left=='/' && (right=='-' || right=='+') { put_char(&mut grid, width, height, x,y, '-'); put=true; }
        if !put && right=='/' && (left=='-' || left=='+') { put_char(&mut grid, width, height, x,y, '-'); put=true; }
        if !put && left=='\\' && (right=='-' || right=='+') { put_char(&mut grid, width, height, x,y, '-'); put=true; }
        if !put && right=='\\' && (left=='-' || left=='+') { put_char(&mut grid, width, height, x,y, '-'); put=true; }

        if !put { println!("error at {} {}", x,y); }
      }
    }
  }
  
  print_grid(&grid, width, height);

  println!("num carts {}",carts.len()); 

  let mut collidedcarts = carts.len();

  loop
  {
    // sort carts
    carts.sort_by_key(|c| c.y * width + c.x);

    for i in 0..carts.len()
    {
      if !carts[i].collided
      {
        let ch = get_char(&grid, width, height, carts[i].x, carts[i].y);

        if ch=='-' && carts[i].dir=='>' { carts[i].x+=1; }
        if ch=='-' && carts[i].dir=='<' { carts[i].x-=1; }
        if ch=='|' && carts[i].dir=='v' { carts[i].y+=1; }
        if ch=='|' && carts[i].dir=='^' { carts[i].y-=1; }
        
        if ch=='\\' && carts[i].dir=='>' { carts[i].x+=1; }
        if ch=='\\' && carts[i].dir=='<' { carts[i].x-=1; }
        if ch=='\\' && carts[i].dir=='v' { carts[i].y+=1; }
        if ch=='\\' && carts[i].dir=='^' { carts[i].y-=1; }
        
        if ch=='/' && carts[i].dir=='>' { carts[i].x+=1; }
        if ch=='/' && carts[i].dir=='<' { carts[i].x-=1; }
        if ch=='/' && carts[i].dir=='v' { carts[i].y+=1; }
        if ch=='/' && carts[i].dir=='^' { carts[i].y-=1; }

        if ch=='+' && carts[i].dir=='^' { carts[i].y-=1; }
        if ch=='+' && carts[i].dir=='v' { carts[i].y+=1; }
        if ch=='+' && carts[i].dir=='<' { carts[i].x-=1; }
        if ch=='+' && carts[i].dir=='>' { carts[i].x+=1; }

        // do turns
        let nextch = get_char(&grid, width, height, carts[i].x, carts[i].y);
        if nextch=='\\' && carts[i].dir=='>' { carts[i].dir='v'; }
        else if nextch=='\\' && carts[i].dir=='<' { carts[i].dir='^'; }
        else if nextch=='\\' && carts[i].dir=='v' { carts[i].dir='>'; }
        else if nextch=='\\' && carts[i].dir=='^' { carts[i].dir='<'; }
        
        else if nextch=='/' && carts[i].dir=='>' { carts[i].dir='^'; }
        else if nextch=='/' && carts[i].dir=='<' { carts[i].dir='v'; }
        else if nextch=='/' && carts[i].dir=='v' { carts[i].dir='<'; }
        else if nextch=='/' && carts[i].dir=='^' { carts[i].dir='>'; }

        if nextch=='+'
        {
          // turn left
          if nextch=='+' && carts[i].state==0 && carts[i].dir=='>' { carts[i].dir='^'; } // left
          else if nextch=='+' && carts[i].state==0 && carts[i].dir=='<' { carts[i].dir='v'; } // left
          else if nextch=='+' && carts[i].state==0 && carts[i].dir=='v' { carts[i].dir='>'; } // left
          else if nextch=='+' && carts[i].state==0 && carts[i].dir=='^' { carts[i].dir='<'; } // left

          // turn right
          else if nextch=='+' && carts[i].state==2 && carts[i].dir=='>' { carts[i].dir='v'; } // right
          else if nextch=='+' && carts[i].state==2 && carts[i].dir=='<' { carts[i].dir='^'; } // right
          else if nextch=='+' && carts[i].state==2 && carts[i].dir=='v' { carts[i].dir='<'; } // right
          else if nextch=='+' && carts[i].state==2 && carts[i].dir=='^' { carts[i].dir='>'; } // right

          carts[i].state+=1;
          if carts[i].state==3 { carts[i].state=0; }
        }

        for k in 0..carts.len()
        {
          if k != i && (carts[i].x==carts[k].x) && (carts[i].y==carts[k].y) && !carts[k].collided
          {
            println!("collision {} {}", carts[i].x, carts[i].y);
  
            carts[i].collided=true;
            carts[k].collided=true;
            collidedcarts-=2;
            break;
          }
        }
      }     
    }

    if collidedcarts==1
    {
      for i in 0..carts.len()
      {
        if !carts[i].collided
        {
          println!("last cart {} {}", carts[i].x, carts[i].y);
        }
      }
      break;
    }
  }

  print_grid_with_carts(&grid, &carts, width, height);
}
