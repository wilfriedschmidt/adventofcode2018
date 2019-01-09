use util::*;

pub fn go(filename:&str)
{
  let payload:Vec<u8> = readfile(filename);
  let inputstr:String = String::from_utf8(payload).unwrap();
 
  let chars:Vec<char> = inputstr.chars().collect();

  let letters = " abcdefghijklmnopqrstuvwxyz";
  for l in 0..27
  {
    let mychar:char = letters.chars().nth(l).unwrap();
    println!("char {}", mychar);

    let mut units:[Vec<char>;2] = [Vec::new(),Vec::new()];

    let mut numunits = 0;
    let mut active = 0;

    units[active].resize(chars.len()-1,' ');
    for i in 0..chars.len()-1
    { 
      if (chars[i] != mychar) && (chars[i] != mychar.to_uppercase().nth(0).unwrap())
      {
        units[active][numunits] = chars[i];
        numunits+=1;
      }
    }

    units[1-active].resize(numunits,' ');
    
    loop
    {
      let mut removed = false;
      let mut sourceindex=0;
      let mut destindex=0;

      loop
      {
        let a:char = units[active][sourceindex];
        let b:char = units[active][sourceindex+1];

        if (a != b) && (a==b.to_uppercase().nth(0).unwrap() || a.to_uppercase().nth(0).unwrap()==b)
        {
          removed = true;
          sourceindex+=1;
        }
        else
        {
          units[1-active][destindex] = a;
          destindex+=1;
        }

        sourceindex+=1;
        if sourceindex >= numunits-1
        {
          units[1-active][destindex] = b;
          destindex+=1;
          break;
        }
      }

      active = 1-active;
      numunits = destindex;

      if !removed
      {
        println!("final len {}", numunits);
        break;
      }
    }
  }
}
