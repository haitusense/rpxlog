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
pub fn rpxlog_sumally(path:&str) -> Robj {
  match ffi::ffi_result(||{
    let df = rpxlog_core::sumally(path)?;
  
    let columns = df.get_columns();
    let key: Vec<&str> = columns[0].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let count: Vec<u32> = columns[1].u32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let dst = data_frame!(
      key = key,
      count = count
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


#[extendr]
pub fn rpxlog_header(path:&str) -> Robj {
  match ffi::ffi_result(||{
    let val = rpxlog_core::header(path)?;
    let src = serde_yaml::to_string(&val)?;
    let dst = match R!("yaml::yaml.load({{ src }})") {
      Ok(n) => n,
      Err(e) => anyhow::bail!(format!("{:?}",e))
    };
    Ok(dst)
  }){
    Ok(n) => n,
    Err(e) => {
      println!("{}", format!("{e:?}").red());
      Robj::from(())
    }
  }
}


#[extendr]
pub fn rpxlog_ptr(path:&str, key:&str) -> Robj {
  match ffi::ffi_result(||{
    let df = rpxlog_core::ptr(path, key)?;
  
    let columns = df.get_columns();
    let cnt: Vec<i32> = columns[0].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let key: Vec<&str> = columns[1].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let result: Vec<f64> = columns[2].f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let unit: Vec<&str> = columns[3].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let real: Vec<f64> = columns[4].f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let num_test: Vec<i32> = columns[5].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let x: Vec<i32> = columns[6].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let y: Vec<i32> = columns[7].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();

    let dst = data_frame!(
      cnt=cnt,
      key=key,
      result=result,
      unit=unit,
      real=real,
      num_test=num_test,
      x=x,
      y=y
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

#[extendr]
pub fn rpxlog_dtr(path:&str, key:&str) -> Robj {
  match ffi::ffi_result(||{
    let df = rpxlog_core::dtr(path, key)?;
  
    let columns = df.get_columns();
    let cnt: Vec<i32> = columns[0].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let result: Vec<&str> = columns[1].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let num_test: Vec<i32> = columns[2].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let x: Vec<i32> = columns[3].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    let y: Vec<i32> = columns[4].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
  
    let dst = data_frame!(
      cnt=cnt,
      result=result,
      num_test=num_test,
      x=x,
      y=y
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
  fn rpxlog_sumally;
  fn rpxlog_header;
  fn rpxlog_ptr;
  fn rpxlog_dtr;
}
