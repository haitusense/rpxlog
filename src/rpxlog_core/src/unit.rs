use anyhow::Context as _;

use regex::Regex;

pub fn unit_to_f64(unit:&str) -> anyhow::Result<f64> {
  Ok(match unit.trim() {    
    "GV" => 1e9,
    "MV" => 1e6,
    "kV" => 1e3,
    "V" => 1e0,
    "mV" => 1e-3,
    "uV" => 1e-6,
    "nV" => 1e-9,
    "pV" => 1e-12,
    "fV" => 1e-15,
    
    "GA" => 1e9,
    "MA" => 1e6,
    "kA" => 1e3,
    "A" => 1e0,
    "mA" => 1e-3,
    "uA" => 1e-6,
    "nA" => 1e-9,
    "pA" => 1e-12,

    "G" => 1e9,
    "M" => 1e6,
    "k" => 1e3,
    "" => 1e0,
    "m" => 1e-3,
    "u" => 1e-6,
    "n" => 1e-9,
    "p" => 1e-12,

    _ => 0f64
  })
}

pub fn value_with_unit_to_f64(src:&str) -> anyhow::Result<f64> {
  let re = Regex::new(r"([\-.0-9]+)([ a-zA-Z]*)")?;
  let dst = re.captures(src).context("caps err")?;

  let val : f64 = dst.get(1).unwrap().as_str().parse()?;
  let unit = dst.get(2).unwrap().as_str();

  Ok(val * unit_to_f64(unit)?)
}

pub fn value_with_unit(src:&str) -> anyhow::Result<(f64,String,f64)> {
  let re = Regex::new(r"([\-.0-9]+)([ a-zA-Z]*)")?;
  let dst = re.captures(src).context("caps err")?;

  let val : f64 = dst.get(1).unwrap().as_str().parse()?;
  let unit = dst.get(2).unwrap().as_str();

  Ok((val, unit.to_string(), val * unit_to_f64(unit)?))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() -> anyhow::Result<()> {
    let src = "-2.83mA";
    let dst = value_with_unit_to_f64(src)?;
    println!("{dst:?}");

    let src = "-2.83A";
    let dst = value_with_unit_to_f64(src)?;
    println!("{dst:?}");

    Ok(())
  }

}