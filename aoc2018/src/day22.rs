use std::collections::HashMap;
use util::*;

#[derive(Clone)]
struct BigNum
{
  words:[u64;16],
}

impl Default for BigNum
{
  fn default() -> BigNum
  {
    BigNum
    {
      words:[0;16]
    }
  }
}

fn mul_bignum(left:&BigNum, right:&BigNum) -> BigNum
{
  let mut res:BigNum = BigNum::default();

  for i in 0..16
  {
    for j in 0..16
    {
      res.words[i] += left.words[i] * right.words[j];
    }

    // shift carries
    for j in i..15
    {
      res.words[j+1] = (res.words[j+1] + res.words[j]>>16);
      res.words[j] = res.words[j] & 0xffff;
    }
  }

  return res;
}

fn add_bignum(left:&BigNum, right:&BigNum) -> BigNum
{
  let mut res:BigNum = BigNum::default();

  let mut carry = 0;
  for i in 0..16
  {
    res.words[i] = left.words[i] + right.words[i] + res.words[i];

    // shift carries
    for j in i..15
    {
      res.words[j+1] = (res.words[j+1] + res.words[j]>>16); 
      res.words[j] = res.words[j] & 0xffff;
    }
  }

  return res;
}

fn from_u64(val:u64) -> BigNum
{
  let mut res:BigNum = BigNum::default();

  res.words[0] = val & 0xffff;
  res.words[1] = (val>>16) & 0xffff;
  res.words[2] = (val>>32) & 0xffff;
  res.words[3] = (val>>48) & 0xffff;

  return res;
}

fn get_u64(val:&BigNum) -> u64
{
  let mut res = val.words[3];
  res<<=16;
  res|=val.words[2];
  res<<=16;
  res|=val.words[1];
  res<<=16;
  res|=val.words[0];

  return res;
}


#[derive(Clone)]
struct Grid
{
  data:Vec<BigNum>,
  width:i64,
  height:i64,
}

impl Default for Grid
{
  fn default() -> Grid
  {
    Grid
    {
      data:Vec::new(),
      width:0,
      height:0,
    }
  }
}
 
fn put_grid(grid:&mut Grid, x:i64, y:i64, value:BigNum)
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    grid.data[ (y*grid.width + x) as usize ] = value;
  }
}

fn get_grid(grid:&Grid, x:i64, y:i64) -> BigNum
{
  if x>=0 && x<grid.width && y>=0 && y<grid.height
  {
    return grid.data[ (y*grid.width + x) as usize ].clone();
  }
  return BigNum::default();
}

fn print_grid(grid:&Grid)
{
  for y in 0..grid.height
  {
    let mut outstr = String::new();
    for x in 0..grid.width
    {
      let val = get_grid(grid,x,y);
      if val.words[0]==0 { outstr.push('.'); }
      if val.words[0]==1 { outstr.push('='); }
      if val.words[0]==2 { outstr.push('|'); }
    }
    println!("{}",outstr);
  } 
}

fn print_bignum(val:&BigNum) 
{
  let mut outstr:String = String::new();
  for i in 0..16
  {
    outstr.push_str(&val.words[i].to_string());
    outstr.push(',');
  }
  println!("[{}]", outstr); 
}

pub fn go(depth:u64, tx:i64, ty:i64)
{

/*
  let mut a:BigNum = BigNum::default();
  let mut b:BigNum = BigNum::default();
  let mut c:BigNum = BigNum::default();
  
  a.words[0] = 255;
  b.words[0] = 7;
  c = mul_bignum(&a,&b);

  println!("{} {} {} {}", c.words[0].to_string(), c.words[1].to_string(), c.words[2].to_string(), c.words[3].to_string());
*/

  let mut geoindex:Grid = Grid::default();
  geoindex.width = tx;
  geoindex.height = ty;
  geoindex.data.resize((geoindex.width*geoindex.height) as usize, BigNum::default());

  let mut erosion:Grid = Grid::default();
  erosion.width = tx;
  erosion.height = ty;
  erosion.data.resize((erosion.width*erosion.height) as usize, BigNum::default());

  for x in 1..erosion.width
  {
    put_grid(&mut erosion, x, 0, from_u64((x*16807) as u64));
  }
  for y in 1..erosion.width
  {
    put_grid(&mut erosion, 0, y, from_u64((y*48271) as u64));
  }

  for y in 1..erosion.height
  {
    for x in 1..erosion.width
    {
      let top = get_grid(&erosion, x, y-1);
      let left = get_grid(&erosion, x-1, y);

      //println!("{} {}", x,y);
      //print_bignum(&top);
      //print_bignum(&left);

      let res = mul_bignum(&top,&left);
      //print_bignum(&res);
      //println!("{}", get_u64(&res));

      put_grid(&mut erosion, x, y, res);
    }
  }

  for y in 0..geoindex.height
  {
    let mut outstr = String::new();
    for x in 0..geoindex.width
    {
      let er = add_bignum(&get_grid(&erosion, x, y), &from_u64(depth));
      let ermod = get_u64(&er);
      let val = (ermod % 20183) % 3;

      if val==0 { outstr.push('.'); }
      if val==1 { outstr.push('='); }
      if val==2 { outstr.push('|'); }
    }
    println!("{}",outstr);
  }
}
