use std::collections::HashMap;
use util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut sum = 0;
  let mut finalsum = 0;

  let mut mymap = HashMap::new();
  mymap.insert(sum,1); 

  let mut i=0;
  loop
  {
    if lines[i].len()>1
    {
      sum += &lines[i].parse::<i32>().unwrap();

      if mymap.contains_key(&sum)
      {
        *mymap.get_mut(&sum).unwrap()+=1;
      }
      else
      {
        mymap.insert(sum,1);
      }

      if mymap[&sum]==2
      {
        println!("freq2: {}",&sum);
        break; 
      }
    }

    i+=1;
    if i>=lines.len()
    {
      i=0;
      if finalsum==0
      {
        finalsum = sum;
      }
    }
  }

  println!("sum: {}", finalsum);
}
