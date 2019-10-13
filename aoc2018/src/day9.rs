use util::*;

#[derive(Clone)]
struct Node
{
  myindex:usize,
  previndex:usize,
  nextindex:usize,

  value:usize,
}

impl Default for Node
{
  fn default() -> Node
  {
    Node
    {
      myindex:0,
      previndex:0,
      nextindex:0,
      value:0,
    }
  }
}

fn alloc_node(nodes:&mut Vec<Node>, nextnodeindex:usize, value:usize) -> usize
{
  nodes[nextnodeindex].nextindex = nextnodeindex;
  nodes[nextnodeindex].value = value;
  return nextnodeindex+1;
}

fn insert_node(nodes:&mut Vec<Node>, currentnodeindex:usize, newnodeindex:usize)
{
  let nextnodeindex = nodes[currentnodeindex].nextindex;

  nodes[newnodeindex].nextindex = nextnodeindex;
  nodes[newnodeindex].previndex = currentnodeindex;

  nodes[nextnodeindex].previndex = newnodeindex;
  nodes[currentnodeindex].nextindex = newnodeindex;
}

fn remove_node(nodes:&mut Vec<Node>, currentnodeindex:usize) -> usize
{
  let prevnodeindex = nodes[currentnodeindex].previndex;
  let nextnodeindex = nodes[currentnodeindex].nextindex;

  nodes[prevnodeindex].nextindex = nextnodeindex;
  nodes[nextnodeindex].previndex = prevnodeindex;

  return nextnodeindex;
}

fn next_node(nodes:&mut Vec<Node>, currentnodeindex:usize) -> usize
{
  return nodes[currentnodeindex].nextindex;
}

fn prev_node(nodes:&mut Vec<Node>, currentnodeindex:usize) -> usize
{
  return nodes[currentnodeindex].previndex;
}

fn print_debug_nodes(nodes:&Vec<Node>, nodeindex:usize, numtoprint:usize)
{
  let mut outstr = String::new();
  let mut currentnodeindex = nodeindex;

  for i in 0..numtoprint
  {
    outstr.push('[');
    outstr.push_str(&nodes[currentnodeindex].myindex.to_string());
    outstr.push(' ');
    outstr.push_str(&nodes[currentnodeindex].value.to_string());
    outstr.push(' ');    
    outstr.push_str(&nodes[currentnodeindex].previndex.to_string());
    outstr.push(' ');
    outstr.push_str(&nodes[currentnodeindex].nextindex.to_string());
    outstr.push_str("] ");
 
    currentnodeindex = nodes[currentnodeindex].nextindex;
  }

  println!("{}", outstr);
}


fn print_nodes(player:usize, nodes:&Vec<Node>, nodeindex:usize, numtoprint:usize, highlight:usize)
{
  let mut outstr = String::new();
  let mut currentnodeindex = nodeindex;

  for i in 0..numtoprint
  { 
    if currentnodeindex==highlight
    {
      outstr.push('(');
      outstr.push_str(&nodes[currentnodeindex].value.to_string());
      outstr.push(')');
    }
    else
    {
      outstr.push(' ');
      outstr.push_str(&nodes[currentnodeindex].value.to_string());
      outstr.push(' ');
    }

    currentnodeindex = nodes[currentnodeindex].nextindex;
  }

  println!("[{}]:{} {}", player.to_string(), numtoprint, outstr);
}

pub fn go(numplayers:usize, lastmarbleworth:usize)
{
  let mut nodes:Vec<Node> = Vec::new();
  nodes.resize((lastmarbleworth)*4, Node::default());
  let rootnodeindex = 0;
  let mut nextnodeindex=rootnodeindex;
  
  // alloc root
  let mut currentnodeindex = nextnodeindex;
  nextnodeindex = alloc_node(&mut nodes, nextnodeindex, 0);
  
  // alloc insert
  let newnodeindex = nextnodeindex;
  nextnodeindex = alloc_node(&mut nodes, nextnodeindex, 1);
  insert_node(&mut nodes, currentnodeindex, newnodeindex);
  currentnodeindex = newnodeindex;

  let mut playerscores:Vec<usize> = Vec::new();
  playerscores.resize(numplayers,0);

  let mut placedmarble:usize=2;
  let mut currentindex:i32=1;
  let mut player = 1;

  loop
  {

    if placedmarble%23 == 0
    {
      playerscores[player] += placedmarble;

      for i in 0..7
      {
        currentnodeindex = prev_node(&mut nodes, currentnodeindex);
      }

      playerscores[player] += nodes[currentnodeindex].value;
      currentnodeindex = remove_node(&mut nodes, currentnodeindex);
    }
    else
    {
      currentnodeindex = next_node(&mut nodes, currentnodeindex);

      // alloc insert
      let newnodeindex = nextnodeindex;
      nextnodeindex = alloc_node(&mut nodes, nextnodeindex, placedmarble);
      insert_node(&mut nodes, currentnodeindex, newnodeindex);
      currentnodeindex = newnodeindex;
    }

    if placedmarble==lastmarbleworth
    {
      break;
    }

    placedmarble+=1;

    if placedmarble % 1000==0
    {
      println!("marble {}", placedmarble.to_string());
    }

    player+=1;
    if player>=numplayers
    {
      player=0;
    }
  }

  for i in 0..numplayers
  {
    println!("player, {}, score, {}", i+1, playerscores[i]);
  }
}
 
