mod ffi;
use anyhow::Context as _;
use thiserror::Error;
use colored::Colorize;
use extendr_api::prelude::*;

#[derive(Error, Debug)]
pub enum RobjError {
	#[error("cannot convert to robj")]
	CannotConvert(),
}

#[extendr]
pub fn polars_to_robj() -> Robj {
  let df = rpxlog_core::polars_to_robj();
  
  let columns = df.get_columns();

  let a: Vec<i32> = columns[0].i32().unwrap().into_no_null_iter().collect();
  let b: Vec<i32> = columns[1].i32().unwrap().into_no_null_iter().collect();
  let c: Vec<f64> = columns[2].f64().unwrap().into_no_null_iter().collect();
  
  data_frame!(x=a, y=b, value=c) 
}

#[extendr]
pub fn rpxlog_ptr_to_df(path:&str, key:&str) -> Robj {
  match ffi::ffi_result(||{
    let df = rpxlog_core::ptr_to_df(path, key)?;
  
    let columns = df.get_columns();
    let cnt: Vec<i32> = columns[0].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let key: Vec<&str> = columns[1].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let result: Vec<f64> = columns[2].f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let unit: Vec<&str> = columns[3].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let real: Vec<f64> = columns[4].f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
  
    let dst = data_frame!(
      cnt=cnt,
      key=key,
      result=result,
      unit=unit,
      real=real
    );
    Ok(dst)
  }){
    Ok(n) => n,
    Err(e) => {
      println!("{}", format!("{e:?}").red());
      Robj::from(())
    }
  }
}


extendr_module! {
  mod rpxlog;
  fn polars_to_robj;
  fn rpxlog_ptr_to_df;
}
