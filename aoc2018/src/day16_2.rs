use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Entry
{
  before:Vec<i32>,
  inst:Vec<i32>,
  after:Vec<i32>
}

fn cmp_regs(regs1:&Vec<i32>, regs2:&Vec<i32>) -> bool
{
  for i in 0..4
  {
    if regs1[i] != regs2[i]
    {
      return false;
    }
  }

  return true;
}


fn runinst(regs:&mut Vec<i32>, opcode:i32, a:i32, b:i32, c:i32)
{
  // addr
  if opcode==0
  {
    regs[c as usize] = regs[a as usize] + regs[b as usize];
  }

  // addi
  if opcode==1
  {
    regs[c as usize] = regs[a as usize] + b;
  }
  
  // mulr
  if opcode==2
  {
    regs[c as usize] = regs[a as usize] * regs[b as usize];
  }

  // muli
  if opcode==3
  {
    regs[c as usize] = regs[a as usize] * b;
  }

  // banr
  if opcode==4
  {
    regs[c as usize] = regs[a as usize] & regs[b as usize];
  }

  // bani
  if opcode==5
  {
    regs[c as usize] = regs[a as usize] & b;
  }

  // borr
  if opcode==6
  {
    regs[c as usize] = regs[a as usize] | regs[b as usize];
  }

  // bori
  if opcode==7
  {
    regs[c as usize] = regs[a as usize] | b;
  }

  // setr
  if opcode==8
  {
    regs[c as usize] = regs[a as usize];
  }

  // seti
  if opcode==9
  {
    regs[c as usize] = a;
  }

  // gtir
  if opcode==10
  {
    if a > regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // gtri
  if opcode==11
  {
    if regs[a as usize] > b { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // gtrr
  if opcode==12
  {
    if regs[a as usize] > regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // eqir
  if opcode==13
  {
    if a == regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // eqri
  if opcode==14
  {
    if regs[a as usize] == b { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // eqrr
  if opcode==15
  {
    if regs[a as usize] == regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }
}


pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut instructions:Vec<Vec<i32>> = Vec::new();
  let mut inst:Vec<i32> = Vec::new();
  inst.resize(4,0);

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let parts:Vec<&str> = lines[i].split(' ').collect();
      inst[0] = parts[0].parse::<i32>().unwrap();
      inst[1] = parts[1].parse::<i32>().unwrap();
      inst[2] = parts[2].parse::<i32>().unwrap();
      inst[3] = parts[3].parse::<i32>().unwrap();

      instructions.push(inst.clone());
    }
  }

  for i in 0..instructions.len()
  {
    println!("{} {} {} {}", instructions[i][0],instructions[i][1],instructions[i][2],instructions[i][3]);
  }

  let mut basemap = HashMap::new();
  basemap.insert(4,15);
  basemap.insert(13,14);  
  basemap.insert(12,13);
  basemap.insert(6,12);  
  basemap.insert(11,11);
  basemap.insert(2,10);
  basemap.insert(9,9);
  basemap.insert(7,8);
  basemap.insert(5,7);
  basemap.insert(3,6);
  basemap.insert(1,5);
  basemap.insert(10,4);
  basemap.insert(15,2);
  basemap.insert(8,3);
  basemap.insert(0,1);
  basemap.insert(14,0);

  let mut regs:Vec<i32> = Vec::new();
  regs.resize(4,0);

  for i in 0..instructions.len()
  {
    let baseopcode = instructions[i][0];

    if basemap.contains_key(&baseopcode)
    {
      let opcode = basemap[&baseopcode];
      runinst(&mut regs, opcode, instructions[i][1], instructions[i][2], instructions[i][3]);
    }
    else
    {
      println!("error");
    }
  }  

  println!("final regs {} {} {} {}", regs[0],regs[1],regs[2],regs[3]);

}
