mod ffi;
use anyhow::Context as _;
use thiserror::Error;
use colored::Colorize;
use extendr_api::prelude::*;
use polars::prelude::*;
use ffi::RobjArgs;
use std::path::Path;

#[derive(Error, Debug)]
pub enum RobjError {
	#[error("cannot convert to robj")]
	CannotConvert(),
}



fn add(s:&Series) -> anyhow::Result<Robj> {
  Ok(match s.dtype() {
    DataType::Utf8 => {
      let key: Vec<_> = s.utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
      R!("data.frame({{key}})").unwrap()
    },
    DataType::Int32 => {
      let key: Vec<_> = s.i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
      R!("data.frame({{key}})").unwrap()
    },
    DataType::Float64 => {
      let key: Vec<_> = s.f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
      R!("data.frame({{key}})").unwrap()
    },
    _=> panic!("no type")
  })
}

fn adds(dst:Robj,s:&Series) -> anyhow::Result<Robj> {
  Ok(match s.dtype() {
    DataType::Utf8 => {
      let key: Vec<_> = s.utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
      R!("cbind( {{dst}}, {{key}} )").unwrap()
    },
    DataType::UInt32 => {
      let key: Vec<_> = s.u32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
      R!("cbind( {{dst}}, {{key}} )").unwrap()
    },
    DataType::Int32 => {
      let key: Vec<_> = s.i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
      R!("cbind( {{dst}}, {{key}} )").unwrap()
    },
    DataType::Float64 => {
      let key: Vec<_> = s.f64().context(RobjError::CannotConvert()).unwrap().into_no_null_iter().collect();
      R!("cbind( {{dst}}, {{key}} )").unwrap()
    },
    _=> panic!("no type")
  })
}

fn df_to_robj(df: DataFrame) -> anyhow::Result<Robj> {
  let columns = df.get_columns();
  let column_names = df.get_column_names();
  let mut dst = add(&columns[0])?;
  for i in 1..columns.len() {
    dst = adds(dst, &columns[i])?;
  }
  let dst = R!("local({
    df <- {{dst}}
    colnames( df ) <- {{column_names}}
    df
  })").unwrap();
  Ok(dst)
}

#[extendr]
pub fn rpxlog_stdf_to_txt(in_path:&str, out_path:&str) {
  match ffi::ffi_result(||{
    rpxlog_core::stdf::stdf_to_txt(in_path, out_path)?;
    Ok(())
  }){
    Ok(_) => { },
    Err(e) => {
      println!("{}", format!("{e:?}").red());
    }
  }
}


#[extendr]
pub fn rpxlog_sumally(path:&str, args:Robj) -> Robj {
  match ffi::ffi_result(||{
    println!("path = {}", path );
    let del = args.to_char("delimiter").unwrap_or("_");
    println!("delimiter = {}", del );
    let path_vec = rpxlog_core::path_split(path, del)?;
    println!(" -> {:?}", path_vec );
    println!("option a = {}", args.to_real("a").unwrap_or(-1f64) );
    println!("option b = {}", args.to_char("b").unwrap_or("na") );

    let df = rpxlog_core::sumally(path)?;
    let df = df_to_robj(df)?;
    let dst = R!("merge( t(data.frame(path = {{path_vec}} )) , {{df}})").unwrap();
  
    // let columns = df.get_columns();
    // let column_names = df.get_column_names();
    // let key: Vec<&str> = columns[0].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let count: Vec<u32> = columns[1].u32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let dst = data_frame!(
    //   key = key,
    //   count = count
    // );

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
    let dst = df_to_robj(df)?;

    // let columns = df.get_columns();
    // let cnt: Vec<i32> = columns[0].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let key: Vec<&str> = columns[1].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let result: Vec<f64> = columns[2].f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let unit: Vec<&str> = columns[3].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let real: Vec<f64> = columns[4].f64().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let num_test: Vec<i32> = columns[5].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let x: Vec<i32> = columns[6].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let y: Vec<i32> = columns[7].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();

    // let dst = data_frame!(
    //   cnt=cnt,
    //   key=key,
    //   result=result,
    //   unit=unit,
    //   real=real,
    //   num_test=num_test,
    //   x=x,
    //   y=y
    // );
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
    let dst = df_to_robj(df)?;
    // let columns = df.get_columns();
    // let cnt: Vec<i32> = columns[0].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let result: Vec<&str> = columns[1].utf8().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let num_test: Vec<i32> = columns[2].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let x: Vec<i32> = columns[3].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
    // let y: Vec<i32> = columns[4].i32().context(RobjError::CannotConvert())?.into_no_null_iter().collect();
  
    // let dst = data_frame!(
    //   cnt=cnt,
    //   result=result,
    //   num_test=num_test,
    //   x=x,
    //   y=y
    // );
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
  fn rpxlog_stdf_to_txt;
  fn rpxlog_sumally;
  fn rpxlog_header;
  fn rpxlog_ptr;
  fn rpxlog_dtr;
}
