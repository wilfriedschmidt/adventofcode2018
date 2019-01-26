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

pub fn strdiff(str1:&str, str2:&str) -> (i32, usize)
{
  let mut diff=0;
  let mut index=0;

  for i in 0..str1.len()
  {
    if str1.chars().nth(i) != str2.chars().nth(i)
    {
      diff += 1;
      index = i;
    }   
  }

  return (diff, index);
}
