use std::collections::HashMap;

pub struct Node
{
  pub name:char,
  pub linkednodes:Vec<char>,
}

impl Default for Node
{
  fn default() -> Node
  {
    Node
    {
      name:' ',
      linkednodes:Vec::new(),
    }
  }
}

pub fn insert_default(nodes:&mut HashMap<char,Node>, name:char)
{
  if !nodes.contains_key(&name)
  {
    let node = Node {name, ..Default::default()};
    nodes.insert(name,node);
  }
}

pub fn loaddata(lines:&Vec<&str>, nextnodes:&mut HashMap<char,Node>, prevnodes:&mut HashMap<char,Node>, fufilled:&mut HashMap<char,bool>, finishedat:&mut HashMap<char,i32>)
{
  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let this = lines[i].chars().nth(5).unwrap();
      let next = lines[i].chars().nth(36).unwrap();

      if !nextnodes.contains_key(&this)
      {
        let mut linkednodes:Vec<char> = Vec::new();
        linkednodes.push(next);
        let node = Node {name:this, linkednodes};

        nextnodes.insert(this,node);
      }
      else
      {
        nextnodes.get_mut(&this).unwrap().linkednodes.push(next);
      }

      // populate defaults
      insert_default(nextnodes, next);
      insert_default(prevnodes, this);
      insert_default(prevnodes, next);

      fufilled.insert(this, false);
      fufilled.insert(next, false);
      finishedat.insert(this, -1);
      finishedat.insert(next, -1);
    }
  }
}

pub fn linkprevnodes(nextnodes:&mut HashMap<char,Node>, prevnodes:&mut HashMap<char,Node>)
{
  // link previous nodes
  for value in nextnodes.values_mut()
  {
    value.linkednodes.sort();
    for i in 0..value.linkednodes.len()
    {
      let name = value.linkednodes[i];
      insert_default(prevnodes, name);
      prevnodes.get_mut(&name).unwrap().linkednodes.push(value.name);
    }
  }
}

pub fn printnodes(nextnodes:&HashMap<char,Node>, prevnodes:&HashMap<char,Node>)
{
  for (key,value) in prevnodes.iter()
  {
    println!("{}", prevnodes[&key].name);

    let mut nexts = String::new();
    for j in 0..nextnodes[&key].linkednodes.len()
    {
      nexts.push(nextnodes[&key].linkednodes[j]);
      nexts.push_str(", ");
    }
    println!("  nexts:[{}]", nexts);

    let mut prevs = String::new();
    for j in 0..prevnodes[&key].linkednodes.len()
    {
      prevs.push(prevnodes[&key].linkednodes[j]);
      prevs.push_str(", ");
    }
    println!("  prevs:[{}]", prevs);
  }
}
