use std::collections::HashMap;
use util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut twos = 0;
  let mut threes = 0;

  for i in 0..lines.len()
  {
    let chars:Vec<char> = lines[i].chars().collect();
    let mut mymap = HashMap::new();
    
    for j in 0..chars.len()
    {
      if mymap.contains_key(&chars[j])
      {
        *mymap.get_mut(&chars[j]).unwrap() += 1;
      }
      else
      {
        mymap.insert(chars[j],1);
      }
    }

    let mut two = false;
    let mut three = false;

    for (_key, value) in mymap
    {
      if value==2
      {
        two = true;
      }
      if value==3
      {
        three = true;
      }
    }

    if two
    {
      twos+=1;
    }
    if three
    {
      threes+=1;
    }
  }

  println!("checksum {}", twos*threes);

  for i in 0..lines.len()
  {
    for j in i..lines.len()
    {
      if lines[j].len() > 1
      {
        let result = strdiff( lines[i], lines[j] );
        if result.0 == 1
        {
          println!("{}{}", &lines[i][..result.1], &lines[i][result.1+1..] );
        }
      }
    }
  }
}
