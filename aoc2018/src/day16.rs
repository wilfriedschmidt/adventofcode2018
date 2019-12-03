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

  let mut entries:Vec<Entry> = Vec::new();

  let mut i=0;
  loop
  {
    if lines[i].len()>1
    {

      let mut dummy:Vec<i32> = Vec::new();
      dummy.resize(4,0);

      let mut entry:Entry = Entry { before: dummy.clone(), inst: dummy.clone(), after: dummy.clone() };
      entry.before[0] = lines[i].chars().nth(9).unwrap().to_string().parse::<i32>().unwrap();
      entry.before[1] = lines[i].chars().nth(12).unwrap().to_string().parse::<i32>().unwrap();
      entry.before[2] = lines[i].chars().nth(15).unwrap().to_string().parse::<i32>().unwrap();
      entry.before[3] = lines[i].chars().nth(18).unwrap().to_string().parse::<i32>().unwrap();
      i+=1;

      let parts:Vec<&str> = lines[i].split(' ').collect();
      entry.inst[0] = parts[0].parse::<i32>().unwrap();
      entry.inst[1] = parts[1].parse::<i32>().unwrap();
      entry.inst[2] = parts[2].parse::<i32>().unwrap();
      entry.inst[3] = parts[3].parse::<i32>().unwrap();

      i+=1;
      entry.after[0] = lines[i].chars().nth(9).unwrap().to_string().parse::<i32>().unwrap();
      entry.after[1] = lines[i].chars().nth(12).unwrap().to_string().parse::<i32>().unwrap();
      entry.after[2] = lines[i].chars().nth(15).unwrap().to_string().parse::<i32>().unwrap();
      entry.after[3] = lines[i].chars().nth(18).unwrap().to_string().parse::<i32>().unwrap();

      //println!("before {} {} {} {}", entry.before[0], entry.before[1], entry.before[2], entry.before[3]); 
      //println!("inst {} {} {} {}", entry.inst[0], entry.inst[1], entry.inst[2], entry.inst[3]); 
      //println!("after {} {} {} {}", entry.after[0], entry.after[1], entry.after[2], entry.after[3]); 

      entries.push(entry);

      i+=2;
    }
    else
    {
      i+=1;
    }

    if i>= lines.len()
    {
      break;
    }
  }

  // base = y
  // actual = x

  let mut masks:[Vec<i32>;16] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];

  let mut hist:Vec<i32> = Vec::new();
  hist.resize(16*16,0);

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
  let mut inst:Vec<i32> = Vec::new();
  let mut totalopcodesmatched=0;
  for i in 0..entries.len()
  {
    let mut thismatchcount=0;

    inst = entries[i].inst.clone();
    let baseopcode = inst[0];

    let mut mask = 0;

    if basemap.contains_key(&baseopcode)
    {
      regs = entries[i].before.clone();
      inst[0] = basemap[&baseopcode];

      runinst(&mut regs, inst[0], inst[1], inst[2], inst[3]);

      if !cmp_regs(&regs, &entries[i].after)
      {
        println!("error {} {}", baseopcode, inst[0]);
      }

      mask = 1<<inst[0];
    }  
    else
    {
      for opcode in 0..16
      {
        regs = entries[i].before.clone();
        
        inst[0] = (baseopcode + opcode)%16;
        runinst(&mut regs, inst[0], inst[1], inst[2], inst[3]);

        if cmp_regs(&regs, &entries[i].after)
        {
          hist[ (baseopcode * 16 + inst[0]) as usize] += 1;
          thismatchcount+=1;

          mask = mask | (1<<inst[0]);
        }
      }

      if thismatchcount>=3
      {
        totalopcodesmatched+=1;
      }
    }

    masks[baseopcode as usize].push(mask);
  }

  let mut totalmasks:Vec<i32> = Vec::new();
  totalmasks.resize(16,0);

  for base in 0..16
  {
    let mut totalmask = 0xffff;
    for i in 0..masks[base].len()
    {
      totalmask = totalmask & masks[base][i];
    } 

    // apply inverse
    for i in 0..16
    {
      if i != (base as i32)
      {
        if basemap.contains_key(&i)
        {
          let mask = !(1 << basemap[&i]);
          totalmask = totalmask & mask;
        }
      }
    }

    totalmasks[base] = totalmask;

    println!("base {:#02} mask {:#018b}", base, totalmask);
  }

/*
  for base in 0..16
  {
    let mut pivot = totalmasks[base];
    for rest in 0..16
    {
      if base !=rest
      {
        pivot = pivot & totalmasks[rest];
      }
    }

    println!("base {:#02} mask {:#018b}", base, pivot);
  }*/

  for base in 0..16
  {
    let mut outstr:String = String::new();
    for actual in 0..16
    {
      outstr.push_str( &hist[base*16+actual].to_string() );
      outstr.push_str(",  ");
    }

    println!("base {} {}", base, outstr);
  }      

  println!("total opcodes matched {}", totalopcodesmatched);
}
