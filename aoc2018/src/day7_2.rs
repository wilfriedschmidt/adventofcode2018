use std::collections::HashMap;
use util::*;
use day7_util::*;

fn is_ready(ic:i32, node:&Node, fufilled:&HashMap<char,bool>, finishedat:&HashMap<char,i32>) -> bool
{ 
  let mut ready = true;
  for j in 0..node.linkednodes.len()
  {
    let prevnode = node.linkednodes[j];
    if !fufilled[&prevnode]
    {
      ready = false;
    }
    else if ic<finishedat[&prevnode] 
    {
      ready = false;
    }
  }

  return ready;
}

fn get_cost(name:char) -> i32
{
  let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  for j in 0..letters.len()
  {
    if letters.chars().nth(j).unwrap()==name
    {
      return (j+1) as i32;
    }
  }

  return -1;
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut nextnodes = HashMap::new();
  let mut prevnodes = HashMap::new();
  let mut fufilled = HashMap::new();
  let mut finishedat = HashMap::new();

  loaddata(&lines, &mut nextnodes, &mut prevnodes, &mut fufilled, &mut finishedat);

  linkprevnodes(&mut nextnodes, &mut prevnodes);

  printnodes(&nextnodes,&prevnodes);

  let mut retstr = String::new();

  let numworkers = 5;
  let mut workers:Vec<i32> = Vec::new();
  workers.resize(numworkers,-1);

  let mut keys = Vec::new();
  for key in prevnodes.keys()
  {
    keys.push(key);
  }
  keys.sort();


  let mut ic = 0;
  loop
  {
    for i in 0..keys.len()
    {
      let node = &prevnodes[keys[i]];

      if !fufilled[&node.name]
      {
        if is_ready(ic, &node, &fufilled, &finishedat)
        {
          let cost = get_cost(node.name);
          for j in 0..workers.len()
          {
            if workers[j] <= ic
            {
              workers[j] = ic + 60 + cost as i32;
              *fufilled.get_mut(&node.name).unwrap()=true;
              *finishedat.get_mut(&node.name).unwrap()=ic + 60 + cost as i32;
        
              retstr.push(node.name);
              break;
            }
          }
        }
      }
    }

    let mut allfufilled = true;
    let mut allfinished = true;
    for value in prevnodes.values()
    {
      if !fufilled[&value.name]
      {
        allfufilled = false;
      }

      if ic < finishedat[&value.name]
      {
        allfinished = false;
      }

      if !allfinished && !allfufilled
      {
        break;
      }
    }

    if allfinished && allfufilled
    {
      break;
    }

    ic+=1;
    
  }

  println!("{} {}", retstr, ic);

}
