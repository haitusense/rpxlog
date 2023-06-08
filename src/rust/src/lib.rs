use anyhow::Context as _;
use extendr_api::prelude::*;


#[extendr]
pub fn polars_to_robj() -> Robj {
  let df = rpxlog_core::polars_to_robj();
  
  let columns = df.get_columns();

  let a: Vec<i32> = columns[0].i32().unwrap().into_no_null_iter().collect();
  let b: Vec<i32> = columns[1].i32().unwrap().into_no_null_iter().collect();
  let c: Vec<f64> = columns[2].f64().unwrap().into_no_null_iter().collect();
  
  data_frame!(x=a, y=b, value=c) 
}

extendr_module! {
  mod rpxlog;
  fn polars_to_robj;
}
