use util::*;

struct Node
{
  index:usize,
  children:Vec<Node>,
  metadata:Vec<i32>,
}

fn readnode(values:&Vec<i32>, inindex:usize) -> Node
{
  let mut index = inindex;

  let numchildren = values[index];
  index+=1;
  let nummetadata = values[index];
  index+=1;

  println!("node {} {} {}", inindex, numchildren, nummetadata);

  let mut children:Vec<Node> = Vec::new();

  for i in 0..numchildren
  {
    let node = readnode(&values, index);

    index = node.index;
    children.push(node);
  }

  let mut metadata:Vec<i32> = Vec::new();
  for i in 0..nummetadata
  {
    metadata.push(values[index]);
    index+=1;
  }

  let node = Node { index, children, metadata };
  return node;
}

fn printnode(node:&Node, indent:usize)
{
  let mut indentstr = String::new();
  for i in 0..indent
  {
    indentstr.push(' ');
  }

  println!("{}node {} {}", indentstr, node.children.len(), node.metadata.len());

  for i in 0..node.children.len()
  {
    printnode(&node.children[i], indent+1);
  }

  let mut metastr = String::new();
  for i in 0..node.metadata.len()
  {
    metastr.push_str(&node.metadata[i].to_string());
    metastr.push(' ');
  }
  println!("{}[ {} ]", indentstr, metastr);
}

fn summetas(node:&Node) -> i32
{
  let mut sum:i32 = 0;
  for i in 0..node.children.len()
  {
    sum += summetas(&node.children[i]);
  }
  
  for i in 0..node.metadata.len()
  {
    sum += node.metadata[i];
  }
  return sum;
}

fn complexsum(node:&Node) -> i32
{
  if node.children.len()==0
  {
    let mut sum:i32 = 0;
    for i in 0..node.metadata.len()
    {
      sum += node.metadata[i];
    }
    return sum;
  }
  else
  {
    let mut sum:i32 = 0;
    for i in 0..node.metadata.len()
    {
      let index = node.metadata[i];
      if index>0 && index <= node.children.len() as i32
      {
        sum += complexsum( &node.children[ (index-1) as usize ] );
      }
    }
    return sum;
  }
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();

  let parts:Vec<&str> = payloadstr[..payloadstr.len()-1].split(' ').collect();

  let mut values:Vec<i32> = Vec::new();

  for i in 0..parts.len()
  {
    if parts[i].chars().nth(0).unwrap().is_digit(10)
    {
      let value = parts[i].parse::<i32>().unwrap();
      values.push(value);
    }
  }

  for i in 0..values.len()
  {
    println!("{}", values[i]);
  }

  let root = readnode(&values, 0);
  printnode(&root,0);

  println!("meta sum {}", summetas(&root));

  println!("sum {}", complexsum(&root));
}
