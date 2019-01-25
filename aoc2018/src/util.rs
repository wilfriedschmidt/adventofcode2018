use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn readfile(filename:&str) -> Vec<u8>
{
  let mut file = match File::open(&filename)
  {
    Err(why) => panic!("couldn't open {} because: {}", filename, why.description()),
    Ok(file) => file,
  };

  let mut payload = Vec::new();
  match file.read_to_end(&mut payload)  
  {
    Err(why) => panic!("couldn't read {} because: {}", filename, why.description()),
    Ok(payload) => payload,
  };

  return payload;
}

pub struct Coord
{
  pub x:i32,
  pub y:i32,
}

pub fn printgrid(grid:&Vec<i32>, gridwidth:i32)
{
  let mut count=0;
  let mut outstr:String = String::new();
  for i in 0..grid.len()
  {    
    let mut gstr:String = String::from("  ");
    if grid[i]!=-1
    {
      gstr = grid[i].to_string();
    }
    loop
    {
      if gstr.len()==2
      {
        break;
      }
      gstr.push(' ')
    }
    outstr.push_str(&gstr);
    outstr.push_str(","); 

    count += 1;
    if count >= gridwidth
    {
      println!("{}", outstr);
      outstr = String::new();
      count=0;
    }
  }
}
