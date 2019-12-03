use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Move
{
  movechar:char,
  children:Vec<Move>,
}

fn recurse_moves(moves:&mut Vec<Move>, input:&str, index:usize) -> usize 
{

  let mut i = index;

  loop
  {
    let ch = input.chars().nth(i).unwrap();
    if ch=='N' || ch=='E' || ch=='W' || ch=='S' || ch=='|'
    {
      let submove:Move = Move { movechar:ch, children:Vec::new() };
      moves.push(submove);
    }
    else if ch=='('
    {
      let mut submove:Move = Move { movechar:' ', children:Vec::new() };      
      i = recurse_moves(&mut submove.children, input, i+1);
      moves.push(submove);    
    }
    else if ch==')'
    {
      return i;
    }

    i+=1;

    if i>=input.len() { break; };
  }

  return i;
}

fn get_moves(outstr:&mut String, moves:&Vec<Move>)
{
  let mut siblings:Vec<Vec<Move>> = Vec::new();  
  let mut current:Vec<Move> = Vec::new();

  for i in 0..moves.len()
  {
    if moves[i].movechar=='|'
    {
      siblings.push(current);
      current = Vec::new();
    }
    else
    {
      current.push(moves[i].clone());
    }
  }

  siblings.push(current);

  for i in 0..siblings.len()
  {
    outstr.push('[');
    for j in 0..siblings[i].len()
    {
      if siblings[i][j].children.len()>0
      {
        outstr.push('(');
        get_moves(outstr, &siblings[i][j].children);
        outstr.push(')');
      }
      else
      {
        outstr.push(siblings[i][j].movechar);
      }
    }
    outstr.push(']');
  }
}

#[derive(Clone,PartialEq,Eq,Hash)]
struct Coord
{
  x:i32,
  y:i32,
}

struct Grid
{
  data:HashMap<Coord,char>,
  minx:i32,
  maxx:i32,
  miny:i32,
  maxy:i32
}

fn put_char(grid:&mut Grid, here:&Coord, ch:char)
{
  if !grid.data.contains_key(&here) { grid.data.insert(here.clone(), ch); }

  if grid.minx > here.x { grid.minx = here.x; }
  if grid.maxx < here.x { grid.maxx = here.x; }
  if grid.miny > here.y { grid.miny = here.y; }
  if grid.maxy < here.y { grid.maxy = here.y; }
}

fn map_moves(grid:&mut Grid, here:&Coord, moves:&Vec<Move>)
{
  let mut siblings:Vec<Vec<Move>> = Vec::new();  
  let mut current:Vec<Move> = Vec::new();

  for i in 0..moves.len()
  {
    if moves[i].movechar=='|'
    {
      siblings.push(current);
      current = Vec::new();
    }
    else
    {
      current.push(moves[i].clone());
    }
  }

  siblings.push(current);

  let mut current:Coord = here.clone();
  let mut stored:Coord = current.clone();

  for i in 0..siblings.len()
  {
    stored = current.clone();

    for j in 0..siblings[i].len()
    {
      if siblings[i][j].children.len()>0
      {
        map_moves(grid, &current, &siblings[i][j].children);
      }
      else
      {
        let ch = siblings[i][j].movechar;
        if ch=='N'
        {
          current.y-=1;
          put_char(grid, &current, '-');
          current.y-=1;
          put_char(grid, &current, ' ');
        }
        else if ch=='W'
        {
          current.x-=1;
          put_char(grid, &current, '|');
          current.x-=1;
          put_char(grid, &current, ' ');
        }
        else if ch=='E'
        {
          current.x+=1;
          put_char(grid, &current, '|');
          current.x+=1;
          put_char(grid, &current, ' ');
        }
        else if ch=='S'
        {
          current.y+=1;
          put_char(grid, &current, '-');
          current.y+=1;
          put_char(grid, &current, ' ');
        }
      }
    }

    current = stored.clone();
  }
}



#[derive(Clone)]
struct MapGrid
{
  data:Vec<char>,
  width:i32,
  height:i32,
}

impl Default for MapGrid
{
  fn default() -> MapGrid
  {
    MapGrid
    {
      data:Vec::new(),
      width:0,
      height:0,
    }
  }
}

fn put_ch(grid:&mut MapGrid, x:i32, y:i32, value:char)
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    grid.data[ (y*grid.width + x) as usize ] = value;
  }
}

fn get_ch(grid:&MapGrid, x:i32, y:i32) -> char
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    return grid.data[ (y*grid.width + x) as usize ];
  }
  return '#';
}

fn print_grid(grid:&MapGrid)
{
  for y in 0..grid.height
  {
    let mut outstr = String::new();
    for x in 0..grid.width
    {
      outstr.push(get_ch(grid,x,y));
    }
    println!("{}",outstr);
  } 
}

fn dist(x:i32, y:i32, dest:&Coord) -> i32
{
  return (dest.x - x).abs() + (dest.y - y).abs();
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

fn try_path_to(grid:&MapGrid, source:&Coord, dest:&Coord) -> Vec<Coord>
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
    
    let xd:[i32;4] = [0,-1,1,0];
    let yd:[i32;4] = [-1,0,0,1];
    for i in 0..4
    {
      let x = next.coord.x + xd[i];
      let y = next.coord.y + yd[i];

      if get_ch(&tempgrid,x,y) == ' ' || get_ch(&tempgrid,x,y) == '|' || get_ch(&tempgrid,x,y) == '-'
      {
        // block it off
        put_ch(&mut tempgrid, x, y, 'X');

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

fn count_doors(grid:&MapGrid, path:&Vec<Coord>) -> i32
{
  let mut doors=0;
  for i in 0..path.len()
  {
    let x = path[i].x;
    let y = path[i].y;
    let ch = get_ch(&grid, x,y);
    if ch=='|' || ch=='-' { doors+=1; }
  }

  return doors;
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();

  let mut moves:Vec<Move> = Vec::new();
  recurse_moves(&mut moves, &payloadstr, 0);
  
  println!("parsing");
  //let mut outstr:String = String::new();
  //get_moves(&mut outstr, &moves);
  //println!("{}", outstr);

  println!("griding");
  let mut grid:Grid = Grid { data:HashMap::new(), minx:1000000, maxx:0, miny:1000000, maxy:0 };
  let origin:Coord = Coord { x:0, y:0 };

  put_char(&mut grid, &origin, ' ');  
  map_moves(&mut grid, &origin, &moves);
  grid.minx-=1;
  grid.maxx+=2;
  grid.miny-=1;
  grid.maxy+=2;

  let mut map:MapGrid = MapGrid::default();

  map.width = grid.maxx-grid.minx;
  map.height = grid.maxy-grid.miny;

  println!("width {} height {}", map.width, map.height);

  map.data.resize((map.width*map.height) as usize,'#');
 
  for my in 0..map.width
  {
    for mx in 0..map.height
    {
      let tempcoord:Coord = Coord { x:mx+grid.minx, y:my+grid.miny };
      if grid.data.contains_key(&tempcoord)
      {
        put_ch(&mut map, mx, my, grid.data[&tempcoord]);
      }    
    }
  }

  let source = Coord {x:-grid.minx, y:-grid.miny};

  //print_grid(&map);

  let mut maxdoors=0;
  let mut numrooms=0;

  println!("counting doors");  

  let mut numcoord=0;

  for y in 0..map.height
  {
    for x in 0..map.width
    {
      numcoord+=1;
      if numcoord%1000==0 { println!("coord {} maxdoors {} numrooms {}", numcoord, maxdoors, numrooms); }

      let dest = Coord {x,y};
      
      let ch = get_ch(&map, x,y);
      if ch==' '
      {    
        let path = try_path_to(&map, &source, &dest);
        if path.len()>0
        {
          // count doors
          let doors = count_doors(&map, &path);
          if doors > maxdoors 
          {
            maxdoors = doors;
            //println!("maxdoors {}", maxdoors); 
          }

          if doors>=1000 { numrooms+=1; }
        }
      } 
    }
  }

  println!("max doors {} numrooms {} ", maxdoors, numrooms);
}
