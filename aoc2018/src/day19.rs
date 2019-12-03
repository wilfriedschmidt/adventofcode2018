use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Instruction
{
  opcode:String,
  a:i64,
  b:i64,
  c:i64
}

fn runinst(regs:&mut Vec<i64>, opcode:&str, a:i64, b:i64, c:i64)
{
  // addr
  if opcode=="addr"
  {
    regs[c as usize] = regs[a as usize] + regs[b as usize];
  }

  // addi
  if opcode=="addi"
  {
    regs[c as usize] = regs[a as usize] + b;
  }
  
  // mulr
  if opcode=="mulr"
  {
    regs[c as usize] = regs[a as usize] * regs[b as usize];
  }

  // muli
  if opcode=="muli"
  {
    regs[c as usize] = regs[a as usize] * b;
  }

  // banr
  if opcode=="banr"
  {
    regs[c as usize] = regs[a as usize] & regs[b as usize];
  }

  // bani
  if opcode=="bani"
  {
    regs[c as usize] = regs[a as usize] & b;
  }

  // borr
  if opcode=="borr"
  {
    regs[c as usize] = regs[a as usize] | regs[b as usize];
  }

  // bori
  if opcode=="bori"
  {
    regs[c as usize] = regs[a as usize] | b;
  }

  // setr
  if opcode=="setr"
  {
    regs[c as usize] = regs[a as usize];
  }

  // seti
  if opcode=="seti"
  {
    regs[c as usize] = a;
  }

  // gtir
  if opcode=="gtir"
  {
    if a > regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // gtri
  if opcode=="gtri"
  {
    if regs[a as usize] > b { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // gtrr
  if opcode=="gtrr"
  {
    if regs[a as usize] > regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // eqir
  if opcode=="eqir"
  {
    if a == regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // eqri
  if opcode=="eqri"
  {
    if regs[a as usize] == b { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }

  // eqrr
  if opcode=="eqrr"
  {
    if regs[a as usize] == regs[b as usize] { regs[c as usize] = 1; }
    else { regs[c as usize] = 0; }
  }
}


pub fn go(filename:&str)
{

  let max=10551300;
  let mut total=1+max;

  for i in 0..max
  {
    for j in 0..max
    {
      let res = i*j;

      if res>max { break; }
      if res==max { total+=j; }      
    }
  }

  println!("{}", total);


/*
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut instructions:Vec<Instruction> = Vec::new();
  let mut ipbinding = 0;

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let parts:Vec<&str> = lines[i].split(' ').collect();
      if lines[i].chars().nth(0).unwrap()=='#'
      {
        ipbinding = parts[1].parse::<usize>().unwrap();
      }
      else
      {
        let inst:Instruction = Instruction {opcode:parts[0].to_string(), a:parts[1].parse::<i64>().unwrap(), b:parts[2].parse::<i64>().unwrap(), c:parts[3].parse::<i64>().unwrap() };
        instructions.push(inst.clone());
      }
    }
  }

  println!("ip binding {}", ipbinding);
  for i in 0..instructions.len()
  {
    println!("{} {} {} {}", instructions[i].opcode, instructions[i].a, instructions[i].b, instructions[i].c );
  }

  let mut regs:Vec<i64> = Vec::new();
  regs.resize(6,0);

  regs[0] = 1;

  //executing ip 13 addi 5 1 5
  //regs 1921 900 1 12 901 900

  // executing ip 13 addi 5 1 5
  //3 10551300 1 12 10551301 3

  let mut ip=0;
  let mut lastip=0;
  let mut instcount=0;

  loop
  {
    instcount+=1;
    if instcount%10000==0 
    { 
      //println!("regs {} {} {} {} {} {}\n", regs[0],regs[1],regs[2],regs[3],regs[4],regs[5]);
      //println!("{}", instcount); 
    };

    if ip>=instructions.len()
    {
      //println!("executing ip {} {} {} {} {}", lastip, instructions[lastip].opcode, instructions[lastip].a, instructions[lastip].b, instructions[lastip].c);
      println!("out of bounds");
      break;
    }
   
    regs[ipbinding] = ip as i64;

    let mut prev = regs[0];

    runinst(&mut regs, &instructions[ip].opcode, instructions[ip].a, instructions[ip].b, instructions[ip].c);
    
    
    //if prev != regs[0]
    //if instcount<60000 //>6480000
    //{
      //println!("executing ip {} {} {} {} {}", ip+1, instructions[ip].opcode, instructions[ip].a, instructions[ip].b, instructions[ip].c); 
      //println!("regs {} {} {} {} {} {}\n", regs[0],regs[1],regs[2],regs[3],regs[4],regs[5]);
    //}

    lastip = ip;
    ip = regs[ipbinding] as usize;

    ip+=1;
  }  

  println!("final regs {} {} {} {} {} {}", regs[0],regs[1],regs[2],regs[3],regs[4],regs[5]);*/
}
