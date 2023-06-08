// use anyhow::Context as _;
use polars::prelude::*;

pub fn polars_to_robj() -> DataFrame {
  let df = df!(
    "x" => &[1,2,3],
    "y" =>  &[4,5,6],
    "value" =>  &[1f64,1.2f64,1.4f64],
  ).unwrap();
  
  df
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() -> anyhow::Result<()> {
    let dst = polars_to_robj();
    println!("{:?}", dst);
    Ok(())
  }

}
