mod txt;
mod stdf;
pub mod unit;
use anyhow::{Context as _, bail};
use thiserror::Error;
use polars::prelude::*;

#[derive(Error, Debug)]
pub enum SystemError {
	#[error("not implemented")]
	NotImplemented(),
}

pub fn polars_to_robj() -> DataFrame {
  let df = df!(
    "x" => &[1,2,3],
    "y" =>  &[4,5,6],
    "value" =>  &[1f64,1.2f64,1.4f64],
  ).unwrap();
  
  df
}

pub fn ptr_to_df(path:&str, key:&str) -> anyhow::Result<DataFrame> {
  let text_pathbuf = std::path::PathBuf::from(path);
  let ext_string = text_pathbuf.extension().context(SystemError::NotImplemented())?.to_string_lossy().into_owned();
  match ext_string.as_str() {
    "stdf" => stdf::stdf_ptr_to_robj(path, key),
    "txt" => txt::txt_ptr_to_robj(path, key),
    _=> anyhow::bail!(SystemError::NotImplemented())
  }
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

  #[test]
  fn it_works_stdf() -> anyhow::Result<()> {
    let df = ptr_to_df("../../sample/1.stdf", "OS_VCC.VDD12L")?;
    println!("{:?}",df);
    // let df = ptr_to_df("../../sample/1.txt", "OS_VCC.VDD12L")?;
    // println!("{:?}",df);

    let columns = df.get_columns();
    let a : Vec<_> = columns[1].utf8().context("a")?.into_no_null_iter().collect();

    println!("{:?}",a);

    Ok(())

  }
  

}
