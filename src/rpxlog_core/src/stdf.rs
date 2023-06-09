use anyhow::{Context as _, bail};
use polars::prelude::*;
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};

/* https://docs.rs/rust-stdf/latest/rust_stdf/ */

/*
  ATR Audit Trail Record
  BPS Begin Program Section Record
* DTR Datalog Text Record
  EPS End Program Section Record 
  FAR File Attributes Record
  FTR Functional Test Record
  GDR Generic Data Record
  HBR Hardware Bin Record
+ MIR Master Information Record
  MPR Multiple-Result Parametric Record
  MRR Master Results Record
  PCR Part Count Record
  PGR Pin Group Record
  PIR Part Information Record
  PLR Pin List Record
  PMR Pin Map Record
* PRR Part Results Record
* PTR Parametric Test Record
  RDR Retest Data Record
  SBR Software Bin Record
+ SDR Site Description Record
  TSR Test Synopsis Record
  WCR Wafer Configuration Record
  WIR Wafer Information Record
  WRR Wafer Results Record

+ : include SirialNum 

sample dataの構成

  1       : FAR(FAR { cpu_type: 2, stdf_ver: 4 })
  2       : ATR(ATR { mod_tim: 1682479600, cmd_line: "RTDF2STDRv4 1.0;AITest V2.3.1 x32 build-10 [20221018153" })
  3       : MIR()
  4       : SDR
  5 - 134 : PMR { pmr_indx: 18, chan_typ: 3, chan_nam: "D4.57", phy_nam: "D4", log_nam: "CH_VREFN_B1", head_num: 1, site_num: 0 }
135 - 205 : PGR(PGR { grp_indx: 32778, grp_nam: "ADC_config_test", indx_cnt: 7, pmr_indx: [69, 70, 24, 46, 48, 49, 35] })
206 - 207 : WCR(WCR { wafr_siz: 0.0, die_ht: 0.0, die_wid: 0.0, wf_units: 0, wf_flat: 'L', center_x: -32768, center_y: -32768, pos_x: 'R', pos_y: 'D' })
208       : WIR(WIR { head_num: 1, site_grp: 2, start_t: 1682479600, wafer_id: "HS01892-07F4" })
209       : PIR(PIR { head_num: 1, site_num: 0 })
210       : DTR(DTR { text_dat: "TIR 2 6 2 1002 OS_VCC" })
211 -     : DTR, PTR, PRR...
*/

macro_rules! stdf {
  ($f:expr, $e:expr) => {
    let mut reader = match StdfReader::new($f){
      Ok(n) => n,
      Err(e) => bail!(format!("{}",e))
    };
  
    for rec in reader.get_record_iter().map(|x| x.unwrap())
    {
      $e(rec)
    }
  };
}

pub fn stdf_to_robj() -> anyhow::Result<()> {
  let stdf_path = "../../sample/1.stdf";

  let mut reader = match StdfReader::new(&stdf_path){
    Ok(n) => n,
    Err(e) => bail!(format!("{}",e))
  };

  
  let mut dut_count: u64 = 0;
  let mut continuity_rlt = vec![];

  let rec_types = REC_PIR | REC_PTR | REC_MIR;
  for rec in reader
    .get_record_iter()
    .map(|x| x.unwrap())
    .filter(|x| x.is_type(rec_types))
  {
    match rec {
      StdfRecord::MIR(n) => {
        println!("{:?}", n);
      },
      StdfRecord::PIR(n) => {
        dut_count += 1;
        println!("{} : {:?}", dut_count, n);
      },
      StdfRecord::PTR(ref ptr_rec) => {
        if ptr_rec.test_txt == "continuity test" {
          continuity_rlt.push(ptr_rec.result);
        }
      },
      _ => {}
    }
  }
  println!("Total duts {} \n continuity result {:?}", dut_count, continuity_rlt);
  Ok(())
}

pub fn stdf_to_robj_full() -> anyhow::Result<()> {
  let stdf_path = "../../sample/1.stdf";

  let mut reader = match StdfReader::new(&stdf_path){
    Ok(n) => n,
    Err(e) => bail!(format!("{}",e))
  };

  for rec in reader
    .get_record_iter()
    .map(|x| x.unwrap())
  {
    println!("{:?}", rec);
  }
  Ok(())
}

pub fn stdf_to_robj_full2() -> anyhow::Result<()> {
  let stdf_path = "../../sample/1.stdf";
  let mut cnt = 0;

  stdf!(stdf_path, |rec| {
    cnt += 1; 
    match (1105 < cnt && cnt < 1122, rec) {
      
      (_, StdfRecord::MIR(n)) => {
        println!("{:?}", n);
      },
      (_, StdfRecord::PMR(n)) => {
        // PMR { pmr_indx: 18, chan_typ: 3, chan_nam: "D4.57", phy_nam: "D4", log_nam: "CH_VREFN_B1", head_num: 1, site_num: 0 }
        println!("PMR : {} : {:?}", cnt, n.pmr_indx);
      },
      (_, StdfRecord::PGR(_)) => { },
      (_, StdfRecord::WCR(n)) => { println!("{:?}", n); },
      (_, StdfRecord::WIR(n)) => { println!("{:?}", n); },

      (_, StdfRecord::PIR(n)) => { println!("{} {:?}", cnt, n); },
      (_, StdfRecord::DTR(n)) => { println!("{} {:?}", cnt, n.text_dat); },

      (_, StdfRecord::PTR(n)) => { println!("{} {} {} {:?} {:?}", cnt, n.test_txt, n.result, n.res_scal, n.units); },


      _=> { }
    };
  });
  Ok(())
}

pub fn stdf_ptr_to_robj(path:&str, key:&str) -> anyhow::Result<DataFrame> {
  let mut cnt = -1;
  let mut vec_cnt = Vec::<i32>::new();
  let mut vec_key = Vec::<String>::new();
  let mut vec_result = Vec::<f64>::new();
  let mut vec_unit = Vec::<String>::new();
  let mut vec_real = Vec::<f64>::new();
  stdf!(path, |rec| {
    match rec {
      StdfRecord::MIR(n) => { 
        println!("{:?}", n);
      },
      StdfRecord::SDR(n) => { 
        println!("{:?}", n);
      },
      StdfRecord::PIR(_) => {
        cnt += 1;
        // println!("{:?}", n);
      },
      StdfRecord::PTR(n) => { 
        if n.test_txt.contains(key) {
          vec_cnt.push(cnt);
          vec_key.push(n.test_txt);
          vec_result.push(n.result as f64);
          vec_unit.push(n.units.unwrap_or("".to_string()));
          vec_real.push(n.result as f64 * 10f64.powf(n.res_scal.unwrap_or(0i8) as f64));
          // println!("{} : {} = {} [{:?}] ({})", cnt, n.test_txt, n.result, n.units, n.result * 10f32.powf(n.res_scal.unwrap_or(0i8) as f32));
        }
      },
      _=> { }
    };
  });
  let df = df!(
    "cnt" => vec_cnt,
    "key" =>  vec_key,
    "result" =>  vec_result,
    "unit" => vec_unit,
    "real" =>  vec_real,
  )?;
  Ok(df)
}
