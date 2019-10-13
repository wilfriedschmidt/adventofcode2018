use std::collections::HashMap;
use util::*;
use day7_util::*;

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

  // collect working set
  let mut workingset = Vec::new();
  for value in prevnodes.values_mut()
  {
    if value.linkednodes.len()==0
    {
      workingset.push(value.name);
    }
  }

  printnodes(&nextnodes,&prevnodes);

  let mut retstr = String::new();

  loop
  {
    workingset.sort();
    workingset.reverse();

    let name = workingset.pop().unwrap();
    *fufilled.get_mut(&name).unwrap()=true;
    retstr.push(name);

    for i in 0..nextnodes[&name].linkednodes.len()
    {
      let nextnode = nextnodes[&name].linkednodes[i];

      let mut ready = true;
      for j in 0..prevnodes[&nextnode].linkednodes.len()
      {
        let prevnode = prevnodes[&nextnode].linkednodes[j];
        if !fufilled[&prevnode]
        {
          ready = false;
        }
      }

      if ready
      {
        workingset.push(nextnode);
      }
    }

    if workingset.len()==0
    {
      break;
    }
  }

  println!("{}", retstr);

}
