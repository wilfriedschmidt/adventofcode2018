#[derive(Clone)]
pub struct Grid<T>
{
  pub data:Vec<T>,
  pub width:i32,
  pub height:i32,
}

impl <T> Default for Grid<T>
{
  fn default() -> Grid<T>
  {
    Grid
    {
      data:Vec::new(),
      width:0,
      height:0,
    }
  }
}

impl <T: std::string::ToString> Grid<T>
{
  pub fn put(&mut self, x:i32, y:i32, value:T)
  {
    if x >= 0 && x < self.width && y >= 0 && y < self.height
    {
      self.data[ (y*self.width + x) as usize ] = value;
    }
  }

  pub fn get(&self, x:i32, y:i32) -> &T
  {
    if x >= 0 && x < self.width && y >= 0 && y < self.height
    {
      return &self.data[ (y*self.width + x) as usize ];
    }
    return &self.data[0]; // this isn't ideal...
  }

  pub fn print(&self)
  {
    for y in 0 .. self.height
    {
      let mut outstr = String::new();    
      for x in 0 .. self.width
      {
        outstr.push_str( &self.get(x,y).to_string() );
      }
      println!("{}",outstr);
    } 
  }
}
