use std::collections::HashMap;
use util::*;
use grid::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();


  let mut totalmins = 1000000000;
  println!("{}", (totalmins - 1980)%28);

  let mut grid:[Grid<char>;2] = [Grid::default(),Grid::default()];
  grid[0].width = lines[0].len() as i32;
  grid[0].height = (lines.len()-1) as i32;
  grid[0].data.resize( (grid[0].width * grid[0].height) as usize,' ');

  grid[1].width = grid[0].width;
  grid[1].height = grid[0].height;
  grid[1].data.resize( (grid[1].width * grid[1].height) as usize,' ');

  println!("width {} height {}", grid[0].width, grid[0].height);

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      for j in 0..lines[i].len()
      {
        grid[0].put(j as i32, i as i32, lines[i].chars().nth(j).unwrap());
      }
    }
  }

  
  let mut adj:Vec<char> = Vec::new();
  adj.resize(8,' ');

  let mut currentindex=0;

  println!(""); 
  grid[currentindex].print();

  for i in 0..2000
  {
    for y in 0..grid[currentindex].height
    {
      for x in 0..grid[currentindex].width
      {
        let mut this = *grid[currentindex].get(x,y);
        adj[0] = *grid[currentindex].get(x-1, y-1);
        adj[1] = *grid[currentindex].get(x, y-1);
        adj[2] = *grid[currentindex].get(x+1, y-1);
        adj[3] = *grid[currentindex].get(x-1, y);
        adj[4] = *grid[currentindex].get(x+1, y);
        adj[5] = *grid[currentindex].get(x-1, y+1);
        adj[6] = *grid[currentindex].get(x, y+1);
        adj[7] = *grid[currentindex].get(x+1, y+1);

        let mut treecount=0;
        let mut lumbercount=0;
        for j in 0..8
        {
          if adj[j]=='|' {treecount+=1;}
          if adj[j]=='#' {lumbercount+=1;}
        }  

        if this=='.'
        {
          if treecount>=3 { grid[1-currentindex].put(x,y, '|'); }
          else { grid[1-currentindex].put(x,y, '.'); }
        }

        if this=='|'
        {
          if lumbercount>=3 { grid[1-currentindex].put( x,y, '#'); }
          else { grid[1-currentindex].put( x,y, '|'); }
        }

        if this=='#'
        {
          if lumbercount>0 && treecount>0 { grid[1-currentindex].put( x,y, '#'); }
          else { grid[1-currentindex].put( x,y, '.'); }
        }
      }
    }

    currentindex = 1-currentindex;

    let mut treecount=0;
    let mut lumbercount=0;
    for y in 0..grid[currentindex].height
    {
      for x in 0..grid[currentindex].width
      {
        let ch = *grid[currentindex].get(x,y);
        if ch=='|' { treecount+=1; }
        if ch=='#' { lumbercount+=1; }
      }
    }
  
    println!("min {} treecount {} lumbercount {} total {}", i+1, treecount, lumbercount, treecount * lumbercount); 
  }
}
