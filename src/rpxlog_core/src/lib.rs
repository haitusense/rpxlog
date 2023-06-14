mod txt;
mod txt_new;
pub mod stdf;
pub mod unit;
use anyhow::{Context as _};
use thiserror::Error;
use polars::prelude::*;
use serde_yaml::Value;
use std::path::Path;

#[derive(Error, Debug)]
pub enum SystemError {
	#[error("not implemented")]
	NotImplemented(),
}


pub fn sumally(path:&str) -> anyhow::Result<DataFrame> {
  let text_pathbuf = std::path::PathBuf::from(path);
  let ext_string = text_pathbuf.extension().context(SystemError::NotImplemented())?.to_string_lossy().into_owned();
  match ext_string.as_str() {
    "stdf" => stdf::stdf_sumally(path),
    _=> anyhow::bail!(SystemError::NotImplemented())
  }
}

pub fn header(path:&str) -> anyhow::Result<Value> {
  let text_pathbuf = std::path::PathBuf::from(path);
  let ext_string = text_pathbuf.extension().context(SystemError::NotImplemented())?.to_string_lossy().into_owned();
  match ext_string.as_str() {
    "stdf" => stdf::stdf_header(path),
    _=> anyhow::bail!(SystemError::NotImplemented())
  }
}

pub fn ptr(path:&str, key:&str) -> anyhow::Result<DataFrame> {
  let text_pathbuf = std::path::PathBuf::from(path);
  let ext_string = text_pathbuf.extension().context(SystemError::NotImplemented())?.to_string_lossy().into_owned();
  match ext_string.as_str() {
    "stdf" => stdf::stdf_ptr(path, key),
    "txt" => txt::txt_ptr_to_robj(path, key),
    _=> anyhow::bail!(SystemError::NotImplemented())
  }
}

pub fn dtr(path:&str, key:&str) -> anyhow::Result<DataFrame> {
  let text_pathbuf = std::path::PathBuf::from(path);
  let ext_string = text_pathbuf.extension().context(SystemError::NotImplemented())?.to_string_lossy().into_owned();
  match ext_string.as_str() {
    "stdf" => stdf::stdf_dtr(path, key),
    // "txt" => txt::txt_ptr_to_robj(path, key),
    _=> anyhow::bail!(SystemError::NotImplemented())
  }
}


pub fn path_split<'a>(path:&'a str, delimiter:&str) -> anyhow::Result<Vec<&'a str>> {
  let path_buf = Path::new(path);
  println!("path = {}", path );
  println!(" stem = {:?}", path_buf.file_stem() );
  println!(" extension = {:?}", path_buf.extension() );

  println!("delimiter = {}", delimiter );
  let a = path_buf.file_stem().unwrap().to_str().unwrap();
  let v: Vec<&str> = a.split(delimiter).collect();
  // println!(" {:?}", v );
  Ok(v)
}

#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn it_works_stdf() -> anyhow::Result<()> {
    let df = ptr("../../sample/1.stdf", "OS_VCC.VDD12L")?;
    println!("{:?}",df);
    // let df = ptr_to_df("../../sample/1.txt", "OS_VCC.VDD12L")?;
    // println!("{:?}",df);

    let columns = df.get_columns();
    let a : Vec<_> = columns[1].utf8().context("a")?.into_no_null_iter().collect();

    println!("{:?}",a);

    Ok(())

  }
  
  #[test]
  fn it_works_2() -> anyhow::Result<()> {
    let df = ptr("../../sample/1.stdf", "OS_VCC.VDD12L")?;
    println!("{:?}",df);
    
    let columns = df.get_columns();
    for i in columns {
      match i.dtype() {
        DataType::Utf8 => println!("Series is of type Utf8"),
        DataType::Int32 => println!("Series is of type Int32"),
        DataType::UInt32 => println!("Series is of type Int32"),
        DataType::Float64 => println!("Series is of type Float64"),
        _=> anyhow::bail!("no type")

      }
    }
    let a : Vec<_> = columns[1].utf8().context("a")?.into_no_null_iter().collect();

    println!("{:?}",a);

    println!("{:?}",df.get_column_names());

    Ok(())

  }
  

}
