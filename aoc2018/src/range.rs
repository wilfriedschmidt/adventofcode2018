#[derive(Clone)]
pub struct Range
{
  pub min:i32,
  pub max:i32,
}

impl Default for Range
{
  fn default() -> Range
  {
    Range
    {
      min:std::i32::MAX,
      max:0,
    }
  }
}
