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
/*
  let mut r0=16457176;
  let mut r1=0;
  let mut r2=0;
  let mut r3=0;
  let mut r4=0;
  let mut r5=0;

  let mut reset=true;
  let mut instcount=0;
  let mut r5s:HashMap<i64,bool> = HashMap::new();

  loop
  {
    if reset
    {
      r4 = r5 | 65536;  // r4 = r5 or 65536
      r5 = 3935295
    }
    reset=false;

    r2 = r4 & 255;    // r2 = r4 mod 256

    r5 = r5 + r5;             // r5 = r5 * 2;
    r5 = r5 & 16777215;       // r5 = r5 mod 16777215
    r5 = r5 * 65899;          // r5 = r5 * 65899
    r5 = r5 & 16777215;       // r5 = r5 mod 16777215    

    if r4<256
    {
      if r5s.contains_key(&r5)
      {
        println!("found repeat {} instcount {}", r5, instcount+1); 
        break;
      }        
      else
      {
        r5s.insert(r5,true);
      }

      if r5==r0
      {
        break;
      }
      else
      {
        reset = true;
      }
    }
    else
    {
      r2=0;
      loop
      {
        r3 = r2+1;
        r3*=256;
      
        if r3>r4 
        {
          r4 = r2;
          break;
        }
        else
        {
          r2+=1;
        }
      }

      reset = true;
    }

    instcount+=1;
  }
    
  println!("instructions executed {}", instcount);
  println!("final regs {} {} {} {} {} {}", r0, r1, r2, r3, r4, r5);
*/


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
  regs[0] = 0;

  let mut ip=0;
  let mut instcount=0;

  let mut r5s:HashMap<i64,bool> = HashMap::new();

  let mut lastr5 = 0;

  loop
  {
    if ip>=instructions.len()
    {
      println!("out of bounds");
      break;
    }
   
    regs[ipbinding] = ip as i64;
    
    runinst(&mut regs, &instructions[ip].opcode, instructions[ip].a, instructions[ip].b, instructions[ip].c);

    if ip==29
    {   
      if r5s.contains_key(&regs[5])
      {
        println!("found repeat {} last {} instcount {}", regs[5], lastr5, instcount+1); 
        break;
      }        
      else
      {
        lastr5 = regs[5];
        r5s.insert(regs[5],true);
      }
    }

/*
    if ip==29
    {
      println!("inst count {} inst ip {} op:{} a:{} b:{} c:{}", instcount+1, ip+1, instructions[ip].opcode, instructions[ip].a, instructions[ip].b, instructions[ip].c); 
      println!("regs 0:{} 1:{} 2:{} 3:{} 4:{} 5:{}\n", regs[0],regs[1],regs[2],regs[3],regs[4],regs[5]);    
    }*/

    //instcount+=1;
    
    ip = regs[ipbinding] as usize;
    ip+=1;
  }  

  println!("instructions executed {}", instcount);
  println!("final regs {} {} {} {} {} {}", regs[0],regs[1],regs[2],regs[3],regs[4],regs[5]);
}
