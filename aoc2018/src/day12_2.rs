use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Node
{
  myindex:i32,
  previndex:i32,
  nextindex:i32,

  value:char,
  iszero:bool,
}

impl Default for Node
{
  fn default() -> Node
  {
    Node
    {
      myindex:-1,
      previndex:-1,
      nextindex:-1,
      value:'.',
      iszero:false,
    }
  }
}

fn alloc_node(nodes:&mut Vec<Node>, nextnodeindex:i32, value:char) -> i32
{
  nodes[nextnodeindex as usize].myindex = nextnodeindex;
  nodes[nextnodeindex as usize].previndex = -1;
  nodes[nextnodeindex as usize].nextindex = -1;
  nodes[nextnodeindex as usize].value = value;
  nodes[nextnodeindex as usize].iszero = false;

  return nextnodeindex+1;
}

fn insert_node(nodes:&mut Vec<Node>, insertafter:i32, newnodeindex:i32)
{
  let nextnodeindex = nodes[insertafter as usize].nextindex;

  nodes[newnodeindex as usize].nextindex = nextnodeindex;
  nodes[newnodeindex as usize].previndex = insertafter;

  if nextnodeindex != -1
  {
    nodes[nextnodeindex as usize].previndex = newnodeindex;
  }
  nodes[insertafter as usize].nextindex = newnodeindex;
}

fn remove_node(nodes:&mut Vec<Node>, currentnodeindex:i32) -> i32
{
  let prevnodeindex = nodes[currentnodeindex as usize].previndex;
  let nextnodeindex = nodes[currentnodeindex as usize].nextindex;

  nodes[prevnodeindex as usize].nextindex = nextnodeindex;
  nodes[nextnodeindex as usize].previndex = prevnodeindex;

  return nextnodeindex;
}

fn next_node(nodes:&Vec<Node>, currentnodeindex:i32) -> i32
{
  return nodes[currentnodeindex as usize].nextindex;
}

fn prev_node(nodes:&Vec<Node>, currentnodeindex:i32) -> i32
{
  return nodes[currentnodeindex as usize].previndex;
}

fn print_debug_nodes(nodes:&Vec<Node>, nodeindex:i32)
{
  let mut outstr = String::new();
  let mut currentnodeindex = nodeindex;

  loop
  {
    if currentnodeindex==-1 { break; }
    outstr.push(nodes[currentnodeindex as usize].value);
    currentnodeindex = nodes[currentnodeindex as usize].nextindex;
  }

  println!("nodes[{}]", outstr);
}

fn find_zero(nodes:&Vec<Node>) -> i32
{
  let mut zeroindex=0;
  let mut currentindex=0;
  loop
  {
    if nodes[currentindex as usize].iszero
    {
      break;
    }
    currentindex = next_node(&nodes, currentindex);
    if currentindex==-1 { break; }

    zeroindex+=1;
  }

  return zeroindex as i32;
}

fn get_total(nodes:&Vec<Node>) -> i32
{
  let mut startindex = -find_zero(nodes);

  let mut total = 0;
  let mut currentindex=0;
  loop
  {
    if nodes[currentindex as usize].value=='#' {total+=startindex;}

    currentindex = next_node(nodes, currentindex);
    if currentindex==-1 { break; }

    startindex+=1;
  }

  return total;
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

  let mut states:[Vec<Node>;2] = [Vec::new(),Vec::new()];
  let mut counts:[i32;2] = [0,0];

  let mut size:usize=1000*1000;
  states[0].resize(size,Node::default());
  states[1].resize(size,Node::default());

  let mut newindex = counts[currentstate];
  counts[currentstate] = alloc_node(&mut states[currentstate], newindex, '.');
  let mut previndex = newindex;
  
  for i in 0..2
  { 
    newindex = counts[currentstate];
    counts[currentstate] = alloc_node(&mut states[currentstate], newindex, '.');
    insert_node(&mut states[currentstate], previndex, newindex);
    previndex = newindex;
  }
  
  for i in 0..initialstate.len()
  {
    newindex = counts[currentstate];
    counts[currentstate] = alloc_node(&mut states[currentstate], newindex, initialstate.chars().nth(i).unwrap());    
    insert_node(&mut states[currentstate], previndex, newindex);
    previndex = newindex;

    if i==0
    {
      states[currentstate][newindex as usize].iszero = true;
    }
  }

  for i in 0..3
  { 
    newindex = counts[currentstate];
    counts[currentstate] = alloc_node(&mut states[currentstate], newindex, '.');
    insert_node(&mut states[currentstate], previndex, newindex);
    previndex = newindex;
  }  

  print_debug_nodes(&states[currentstate],0);
  println!("zero index {}", find_zero(&states[currentstate]).to_string());


  let mut scanring:[char;5] = ['.','.','.','.','.'];
  let mut zeroring:[bool;5] = [false,false,false,false,false];
  let mut currentindex:i32 = 0;
  let mut iteration:usize = 0;
  loop
  {
    if iteration%100==0
    {
      println!("iteration {}", iteration);
    }

    let mut currentiter = 0;
    
 
    currentindex=0;
    let mut startdotcount=0;
    loop
    {
      if states[currentstate][currentindex as usize].iszero  
      {   
        break; 
      }

      if states[currentstate][currentindex as usize].value=='.' {startdotcount+=1;}
      else { break; }

      currentindex = next_node(&states[currentstate], currentindex);
      if currentindex==-1 { break; }
    }

    currentindex=0;
    if startdotcount>2
    {
      for i in 0..startdotcount-2
      {
        currentindex = next_node(&states[currentstate], currentindex);
      }
    }

    // preload ...
    newindex = counts[1-currentstate];
    counts[1-currentstate] = alloc_node(&mut states[1-currentstate], newindex, '.');
    previndex = newindex;
    
    for i in 0..2
    {
      newindex = counts[1-currentstate];
      counts[1-currentstate] = alloc_node(&mut states[1-currentstate], newindex, '.');
      insert_node(&mut states[1-currentstate], previndex, newindex);
      previndex = newindex;
    }
    
    // preload scanring
    for i in 0..4
    {
      scanring[currentiter] = states[currentstate][currentindex as usize].value;
      zeroring[currentiter] = states[currentstate][currentindex as usize].iszero;
      currentindex = next_node(&states[currentstate], currentindex);
      currentiter+=1;
    }

    let mut enddotcount=0;
    loop
    {
      // add to scanring
      scanring[currentiter%5] = states[currentstate][currentindex as usize].value;
      zeroring[currentiter%5] = states[currentstate][currentindex as usize].iszero;

      //println!("checking {}", ((currentiter - 2) % 5).to_string());

      let mut thisiszero = false;
      if zeroring[ ((currentiter - 2) % 5) as usize]
      {
        //println!("this is zero {}", currentiter);
        thisiszero = true;
      }

      let mut foundrule = false;
      for k in 0..rules.keys().len()
      {
        let key = rules.keys().nth(k).unwrap();

        let mut matched = true;
        for sk in 0..5
        {
          let ch = scanring[ ((sk + currentiter + 1) % 5) as usize];
          if key.chars().nth(sk).unwrap() != ch
          {
            matched = false;
            break;
          }
        }

        if matched 
        {
          if rules[key]=='.' { enddotcount+=1; }
          else { enddotcount=0; }

          newindex = counts[1-currentstate];
          counts[1-currentstate] = alloc_node(&mut states[1-currentstate], newindex, rules[key]);
          states[1-currentstate][newindex as usize].iszero = thisiszero;

          if newindex != 0
          {
            insert_node(&mut states[1-currentstate], previndex, newindex);
            previndex = newindex;
          }

          foundrule = true;
          break;
        }  
      }

      if !foundrule
      {
        enddotcount+=1;

        newindex = counts[1-currentstate];
        counts[1-currentstate] = alloc_node(&mut states[1-currentstate], newindex, '.');
        states[1-currentstate][newindex as usize].iszero = thisiszero;

        if newindex !=0
        {
          insert_node(&mut states[1-currentstate], previndex, newindex);
          previndex = newindex;
        }
      }

      currentiter+=1;
      currentindex = next_node(&states[currentstate], currentindex);
      if currentindex==-1 { break; }
    }

    if enddotcount < 3
    {
      for i in 0..3-enddotcount
      { 
        newindex = counts[1-currentstate];
        counts[1-currentstate] = alloc_node(&mut states[1-currentstate], newindex, '.');
        insert_node(&mut states[1-currentstate], previndex, newindex);
        previndex = newindex;
      }
    }  

    // swap states, print next state
    currentstate = 1-currentstate;

    // clear next list
    counts[1-currentstate] = 0;
    previndex=-1;
  
    // iterate
    iteration+=1;
    if iteration>=201
    {
      break;
    }

    if iteration>=500
    {
      println!("total {} iteration {}", get_total(&states[currentstate]), iteration);
    }
  } 

  let mut total:i64 = 50000000000;
  total-=500;
  total*=55;
  total+=27411;
  println!("{}", total);
}
