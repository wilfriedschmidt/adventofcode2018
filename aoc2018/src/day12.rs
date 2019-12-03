use std::collections::HashMap;
use util::*;

fn print_state(state:&Vec<char>)
{

  let mut start=0;
  let mut end=0;
  for i in 0..state.len()
  {
    if state[i]=='#' 
    {
      if start==0 {start=i;} 
      end = i; 
    }
  }

  let mut outstr = String::new();
  for i in start-3..end+2
  {
    outstr.push(state[i]);
  }

  println!("[{}]", outstr);

}

fn num_to_scan(state:&Vec<char>) -> (usize,usize)
{
  let mut start:usize=0;
  let mut end:usize=0;
  for i in 0..state.len()
  {
    if state[i]=='#' 
    {
      if start==0 {start=i;} 
      end = i; 
    }
  }

  return (start,end);
}

fn count(state:&Vec<char>,size:i32) -> i32
{
  let mut count:i32=0;
  for i in 0..state.len()
  {
    if state[i]=='#' 
    {
      count+= (i as i32 -size/2);
    }
  }

  return count;
}

pub fn go(filename:&str, initialstate:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();


  let mut rules = HashMap::new();

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let rule = &lines[i][..5];
      let outcome = lines[i].chars().nth(9).unwrap();

      rules.insert(rule,outcome);
    }
  }

  let mut currentstate=0;
  let mut states:[Vec<char>;2] = [Vec::new(),Vec::new()];

  let mut size:usize=1000;
  states[0].resize(size,'.');
  states[1].resize(size,'.');

  for i in 0..initialstate.len()
  {
    states[currentstate][i+size/2] = initialstate.chars().nth(i).unwrap();
  }

  print_state(&states[currentstate]);

  let mut iteration:usize = 0;
  loop
  {
    if iteration%1000000==0
    {
      println!("iteration {}", iteration);
    }

    let result = num_to_scan(&states[currentstate]);
    
    for j in result.0-5..result.1+5
    {
      for k in 0..rules.keys().len()
      {
        let key = rules.keys().nth(k).unwrap();

        let mut found = true;
        for sk in 0..5
        {
          if key.chars().nth(sk).unwrap() != states[currentstate][(j+sk) as usize]
          {
            found = false;
            break;
          }
        }

        if found 
        {
          states[1-currentstate][j+2] = rules[key];
        }
      }
    }

    for j in 0..states[currentstate].len()
    {
      states[currentstate][j]='.';
    }

    currentstate = 1-currentstate;
    print_state( &states[currentstate] );

    iteration+=1;
    if iteration>=20
    {
      break;
    }
  } 

  println!("count: {}", count(&states[currentstate],size as i32).to_string()); 
}
