#[derive(Clone)]
pub struct Coord
{
  pub x:i32,
  pub y:i32,
}

impl Default for Coord
{
  fn default() -> Coord
  {
    Coord
    {
      x:0,
      y:0,
    }
  }
}
