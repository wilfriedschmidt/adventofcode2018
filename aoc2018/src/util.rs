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
