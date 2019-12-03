use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct Group
{
  id:i32,
  isimmunesystem:bool,
  numunits:i32,
  hp:i32,
  weaknesses:Vec<String>,
  immuneto:Vec<String>,
  attack:i32,
  damage:String,
  initiative:i32,
  targetid:i32,
  attackedby:i32,
  sort:i32,
}

fn effective_power(group:&Group) -> i32
{
  return group.numunits * group.attack;
}

fn calc_damage(attacker:&Group, defender:&Group) -> i32
{
  let mut isimmune=false;
  for i in 0..defender.immuneto.len()
  {
    if attacker.damage==defender.immuneto[i] { isimmune=true; break; }
  }

  let mut isweak=false;
  for i in 0..defender.weaknesses.len()
  {
    if attacker.damage==defender.weaknesses[i] { isweak=true; break; }
  }

  if isimmune==true { return 0; }
  if isweak==true { return effective_power(attacker)*2; }

  return effective_power(attacker);
}

fn get_index(groups:&Vec<Group>, id:i32) -> usize
{
  for i in 0..groups.len()
  {
    if groups[i].id==id { return i; }
  }
  
  return 0;
}

fn print_groups(groups:&Vec<Group>)
{
  for i in 0..groups.len()
  {
    if groups[i].isimmunesystem { println!("immunesystem:"); }
    else  { println!("infection:"); }

    println!("  id {} num units {} hp {} attack {} damage {} initiative {}", groups[i].id, groups[i].numunits, groups[i].hp, groups[i].attack, groups[i].damage, groups[i].initiative);

    for j in 0..groups[i].immuneto.len()
    {
      println!("    immuneto {}", groups[i].immuneto[j]);
    }  

    for j in 0..groups[i].weaknesses.len()
    {
      println!("    weakto {}", groups[i].weaknesses[j]);
    }  

    println!("    attacking {} attacked by {}", groups[i].targetid, groups[i].attackedby);
  }
}

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let payloadstr:String = String::from_utf8(payload).unwrap();
  let lines:Vec<&str> = payloadstr.split('\n').collect();

  let mut groups:Vec<Group> = Vec::new();
 
  let mut writingimmune=true;
  let mut id:i32=0;

  for i in 0..lines.len()
  {
    if lines[i].len()>1
    {
      let parts:Vec<&str> = lines[i].split(' ').collect();

      if parts[0]=="Infection:" { writingimmune=false; }
      else if parts.len()>2
      {
        let numunits = parts[0].parse::<i32>().unwrap();
        let hp = parts[4].parse::<i32>().unwrap();
        
        let mut attack=0;
        let mut damage:String = String::new();
        for j in 0..parts.len()
        {
          if parts[j]=="does"
          {
            attack = parts[j+1].parse::<i32>().unwrap();
            damage = parts[j+2].to_string();
            break;
          }
        }

        let mut initiative=0;
        for j in 0..parts.len()
        {
          if parts[j]=="initiative"
          {
            initiative = parts[j+1].parse::<i32>().unwrap();
            break;
          }
        }

        let mut weaknesses:Vec<String> = Vec::new();
        let mut immuneto:Vec<String> = Vec::new();

        for j in 0..parts.len()
        {
          if parts[j]=="weak" || parts[j]=="(weak"
          {
            let mut k = j+2;
            loop
            {
              let mut weakness = parts[k];
              let lastchar = weakness.chars().nth(weakness.len()-1).unwrap();
              if lastchar==')' || lastchar==',' || lastchar==';'
              {
                weakness = &weakness[..weakness.len()-1];
              }
              weaknesses.push(weakness.to_string());

              if lastchar==')' || lastchar==';' { break; }
              k+=1;
            }
            break;
          }
        }

        for j in 0..parts.len()
        {
          if parts[j]=="immune" || parts[j]=="(immune"
          {
            let mut k = j+2;
            loop
            {
              let mut immune = parts[k];
              let lastchar = immune.chars().nth(immune.len()-1).unwrap();
              if lastchar==')' || lastchar==',' || lastchar==';'
              {
                immune = &immune[..immune.len()-1];
              }
              immuneto.push(immune.to_string());

              if lastchar==')' || lastchar==';' { break; }
              k+=1;
            }
            break;
          }
        }

        let group:Group = Group { id, isimmunesystem: writingimmune, numunits, hp, weaknesses, immuneto, attack, damage:damage.clone(), initiative, targetid:-1, attackedby:-1, sort:0 };
        groups.push(group);
        id+=1;
      }
    }
  }

  print_groups(&groups);

  let mut basegroups = groups.clone();

  let mut boost=25;
  loop
  {

    println!("boost {}", boost);
    groups = basegroups.clone();

    for i in 0..groups.len()
    {
      if groups[i].isimmunesystem
      {
        groups[i].attack+=boost;
      }
    }

    let mut numinfection=0;
    let mut numimmune=0;  

    let mut prevnuminfection=0;
    let mut prevnumimmune=0;  

    let mut fightcount=0;
    loop
    {
      //println!("\nFIGHT {}", fightcount);
      fightcount+=1;

      let numgroups=groups.len() as i32;
      for i in 0..groups.len()
      {
        groups[i].targetid = -1;
        groups[i].attackedby = -1;
        groups[i].sort = effective_power(&groups[i])*numgroups + groups[i].initiative;
      }
     
      // target selection
      groups.sort_by_key(|x| x.sort);
      groups.reverse();

      for i in 0..groups.len()
      {
        if groups[i].numunits>0
        {
          //println!("id {} sort {} power {} ", groups[i].id, groups[i].sort, effective_power(&groups[i])); 

          let mut maxdamage = -1;
          let mut maxpower = 0;
          let mut maxinit = 0;
          let mut maxdamageid = -1;
          for j in 0..groups.len()
          {
            if i != j && groups[j].attackedby==-1 && groups[j].isimmunesystem != groups[i].isimmunesystem && groups[j].numunits>0
            {
              let damage = calc_damage(&groups[i], &groups[j]);

              if damage !=0
              {
                let attackeepower = effective_power(&groups[j]);          
                if damage > maxdamage
                {
                  maxdamage = damage;
                  maxpower = attackeepower;
                  maxinit = groups[j].initiative;
                  maxdamageid = groups[j].id;
                }
                else if damage==maxdamage
                {
                  if attackeepower > maxpower
                  {
                    maxdamage = damage;
                    maxpower = attackeepower;
                    maxinit = groups[j].initiative;
                    maxdamageid = groups[j].id;
                  }
                  else if attackeepower==maxpower
                  {
                    if groups[j].initiative > maxinit
                    {
                      maxdamage = damage;
                      maxpower = attackeepower;
                      maxinit = groups[j].initiative;
                      maxdamageid = groups[j].id;
                    }
                  }
                }
              }
            }
          }
        
          if maxdamageid !=-1
          {          
            groups[i].targetid = maxdamageid;

            let index = get_index(&groups, maxdamageid);
            groups[index].attackedby = groups[i].id as i32;
          }
        }
      }

      //print_groups(&groups);

      // attack
      groups.sort_by_key(|x| x.initiative);
      groups.reverse();

      for i in 0..groups.len()
      {
        if groups[i].numunits>0 && groups[i].targetid>=0
        {
          let tindex = get_index(&groups, groups[i].targetid);
          let damage = calc_damage(&groups[i], &groups[tindex]);

          //println!("id {} initiative {} attacking {} with damage {}", groups[i].id, groups[i].initiative, groups[tindex].id, damage); 
              
          let mut numkilled = damage / groups[tindex].hp;

          if numkilled >= groups[tindex].numunits { numkilled = groups[tindex].numunits; }
          groups[tindex].numunits -= numkilled;

          /*
          if numkilled > 0
          {
            println!("group {} damage {} killing {} of group {}", groups[i].id, damage, numkilled, groups[tindex].id);
          }*/
        }
      }

      //print_groups(&groups);

      // count armies
      numinfection=0;
      numimmune=0;
      for i in 0..groups.len()
      {
        if groups[i].isimmunesystem && groups[i].numunits>0 { numimmune+=groups[i].numunits; }
        if !groups[i].isimmunesystem && groups[i].numunits>0 { numinfection+=groups[i].numunits; }
      }      

      //println!("numimmune {} numinfection {}", numimmune, numinfection);
      if numimmune==0 || numinfection==0 
      {
        println!("fight {} numimmune {} numinfection {}", fightcount, numimmune, numinfection);
        break; 
      }

      if prevnumimmune==numimmune && prevnuminfection==numinfection
      {
        println!("stalemate {} numimmune {} numinfection {}", fightcount, numimmune, numinfection);
        break;
      }

      prevnumimmune=numimmune;
      prevnuminfection=numinfection;
    }

    if numinfection==0 { break; }

    boost+=1;
  }
}
