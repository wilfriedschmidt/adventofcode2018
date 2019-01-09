use std::collections::HashMap;
use util::*;

struct Entry
{
  id:i32,
  state:i32,
  totalmins:i64,

  min:i64,
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut entries:Vec<Entry> = Vec::new();

  for i in 0..lines.len()
  {
    if lines[i].len() > 1
    {
      //[1518-11-01 23:58] Guard #99 begins shift
      let month = lines[i][6..8].parse::<i64>().unwrap();
      let day = lines[i][9..11].parse::<i64>().unwrap();

      let hour = lines[i][12..14].parse::<i64>().unwrap();
      let min = lines[i][15..17].parse::<i64>().unwrap();

      let totalmins:i64 = month*31*24*60 + day*24*60 + hour*60 + min;

      let statestr = &lines[i][19..];
      let mut state=-1;
      let mut id=-1;
      if statestr.find("Guard")!=None
      {
        let parts:Vec<&str> = statestr.split(' ').collect();
        id = parts[1][1..].parse::<i32>().unwrap();
  
        state = 0;
      }
      if statestr.find("falls")!=None
      {
        state = 1;
      }
      if statestr.find("wakes")!=None
      {
        state = 2;
      }

      let e:Entry = Entry { id, state, totalmins, min };

      entries.push(e);
    }
  }

  entries.sort_by_key(|x| x.totalmins);

  let mut mymap = HashMap::new();
  
  let mut guardtoindex = HashMap::new();
  let mut guardindex:i64 = 0;

  let mut currentid = -1;
  let mut asleepmin = -1;
  for i in 0..entries.len()
  {
    if entries[i].id != -1
    {
      currentid = entries[i].id;

      if !guardtoindex.contains_key(&currentid)
      {
        guardtoindex.insert(currentid,guardindex);
        guardindex+=1;
      }
    }
    else 
    {
      if entries[i].state==1
      {
        asleepmin = entries[i].totalmins;
      }
      else if entries[i].state==2
      {
        let timeasleep = entries[i].totalmins - asleepmin;

        if !mymap.contains_key(&currentid)
        {
          mymap.insert(currentid,timeasleep);
        }
        else
        {
          *mymap.get_mut(&currentid).unwrap() += timeasleep;
        }
      }
      else
      {
        println!("DATA ERROR!!!");
      }
    }
  }

  let mut largestid=0;
  let mut largesttotalmins=0;

  for (key,value) in mymap
  {
    if value>largesttotalmins
    {
      largestid=key;
      largesttotalmins = value;
    }
  }

  println!("longest asleep {}", largestid);   

  let mut totaltotalmins = Vec::new();
  totaltotalmins.resize(60 * guardindex as usize,0);

  let mut highestminindex = 0;
  let mut highestmin = 0;
  let mut highestguardid = 0;

  let mut largestminindex = 0;
  let mut largestmin = 0;

  for i in 0..entries.len()
  {
    if entries[i].id != -1
    {
      currentid = entries[i].id;
    }
    else 
    {
      if entries[i].state==1
      {
        asleepmin = entries[i].min;
      }
      else if entries[i].state==2
      {
        for m in (asleepmin + guardtoindex[&currentid]*60)..(entries[i].min + guardtoindex[&currentid]*60)
        {
          totaltotalmins[m as usize]+=1;

          if totaltotalmins[m as usize]  > highestmin
          {
            highestmin = totaltotalmins[m as usize];
            highestminindex = m - guardtoindex[&currentid]*60;
            highestguardid = currentid;
          }

          if currentid==largestid
          {
            if totaltotalmins[m as usize]  > largestmin
            {
              largestmin = totaltotalmins[m as usize];
              largestminindex = m - guardtoindex[&currentid]*60;
            }
          }
        }
      }
      else
      {
        println!("DATA ERROR!!!");
      }
    }       
  }

  println!("highest min {}", highestminindex);
  println!("highest guardid {}", highestguardid);

  println!("largest min {}", largestminindex);
  println!("largest guardid {}", largestid);
}
