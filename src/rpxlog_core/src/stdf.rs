use anyhow::{Context as _, bail};
use polars::prelude::*;
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};
use serde_yaml::Value;

/* https://docs.rs/rust-stdf/latest/rust_stdf/ */

/* STDF Specification

  ATR Audit Trail Record
  BPS Begin Program Section Record
* DTR Datalog Text Record
  EPS End Program Section Record 
  FAR File Attributes Record
  FTR Functional Test Record
  GDR Generic Data Record
  HBR Hardware Bin Record
  MIR Master Information Record
  MPR Multiple-Result Parametric Record
  MRR Master Results Record
  PCR Part Count Record
  PGR Pin Group Record
* PIR Part Information Record
  PLR Pin List Record
  PMR Pin Map Record
* PRR Part Results Record
* PTR Parametric Test Record
  RDR Retest Data Record
  SBR Software Bin Record
  SDR Site Description Record
* TSR Test Synopsis Record
  WCR Wafer Configuration Record
  WIR Wafer Information Record
  WRR Wafer Results Record

*/

trait ToString {
  fn to_string(&self) -> String;
}

impl ToString for rust_stdf::StdfRecord {
  fn to_string(&self) -> String {
    match self {
      Self::FAR(_) => "FAR",
      Self::ATR(_) => "ATR",
      Self::VUR(_) => "VUR",
      Self::MIR(_) => "MIR",
      Self::MRR(_) => "MRR",
      Self::PCR(_) => "PCR",
      Self::HBR(_) => "HBR",
      Self::SBR(_) => "SBR",
      Self::PMR(_) => "PMR",
      Self::PGR(_) => "PGR",
      Self::PLR(_) => "PLR",
      Self::RDR(_) => "RDR",
      Self::SDR(_) => "SDR",
      Self::PSR(_) => "PSR",
      Self::NMR(_) => "NMR",
      Self::CNR(_) => "CNR",
      Self::SSR(_) => "SSR",
      Self::CDR(_) => "CDR",
      Self::WIR(_) => "WIR",
      Self::WRR(_) => "WRR",
      Self::WCR(_) => "WCR",
      Self::PIR(_) => "PIR",
      Self::PRR(_) => "PRR",
      Self::TSR(_) => "TSR",
      Self::PTR(_) => "PTR",
      Self::MPR(_) => "MPR",
      Self::FTR(_) => "FTR",
      Self::STR(_) => "STR",
      Self::BPS(_) => "BPS",
      Self::EPS(_) => "EPS",
      Self::GDR(_) => "GDR",
      Self::DTR(_) => "DTR",
      Self::ReservedRec(_) => "ReservedRec",
      Self::InvalidRec(_) => "InvalidRec",
    }.to_string()
  }
}

/* STDF vs TXT

[STDF]    [TXT]

FAR/ATR   (file attributes/audit trail)
MIR/SDR   [Header]
|lot_id   Cust_LotId=Foo01
|         Customer_Id=Bar01
|         .
|         .
|         .
PMR/PGR   (pin map/group)
WCR/WIR   (wafer configuration/information)
PIR       TCNT# 0        SITE# 0
PMR
          NO Site  Result  TestName           Signal          Measure   LowLimit  HighLimit   Force
PTR       1002 0   PASS  OS_VCC               VDD33           -0.236 V   -0.700V   -0.050V   -200.000uA
          .
          .
          .
          00   0   PASS  Temp_AF_1  Reg[252-253-254]	
DTR       PixelTest_Time: 104.935158 sec
PRR       -------------------   ------------
|         Site   Fail   Total    Cate   Bin    XCoord   YCoord         TestTime(122868ms)
|         -------------------   ------------
|           0      0       1       1     1      10       1
          .
          .
          .
WRR       (wafer results)
TSR       (test synopsis)
HBR/SBR   (hardware/software bin)
MRR       (master results)


sample dataの構成

  1           : FAR(FAR { cpu_type: 2, stdf_ver: 4 })
  2           : ATR(ATR { mod_tim: 1682479600, cmd_line: "RTDF2STDRv4 1.0;AITest V2.3.1 x32 build-10 [20221018153" })
  3           : MIR
  4           : SDR
  5 - 134     : PMR { pmr_indx: 18, chan_typ: 3, chan_nam: "D4.57", phy_nam: "D4", log_nam: "CH_VREFN_B1", head_num: 1, site_num: 0 }
135 - 205     : PGR(PGR { grp_indx: 32778, grp_nam: "ADC_config_test", indx_cnt: 7, pmr_indx: [69, 70, 24, 46, 48, 49, 35] })
206 - 207     : WCR(WCR { wafr_siz: 0.0, die_ht: 0.0, die_wid: 0.0, wf_units: 0, wf_flat: 'L', center_x: -32768, center_y: -32768, pos_x: 'R', pos_y: 'D' })
208           : WIR(WIR { head_num: 1, site_grp: 2, start_t: 1682479600, wafer_id: "HS01892-07F4" })
209           : PIR(PIR { head_num: 1, site_num: 0 })
210           : DTR(DTR { text_dat: "TIR 2 6 2 1002 OS_VCC" })
211 -         : DTR, PTR, PRR...
247928        : last PRR
247929        : WRR { head_num: 1, site_grp: 2, finish_t: 1682517833, part_cnt: 319, ... }
247930-       : TSR
248960-249001 : HBR / SBR
249002        : PCR { head_num: 1, site_num: 0, part_cnt: 319, rtst_cnt: 0, abrt_cnt: 0, good_cnt: 255, func_cnt: 4294967295 }
249003        : MRR { finish_t: 1682517833, disp_cod: ' ', usr_desc: "", exc_desc: "" }
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

trait PrintAll{
  fn print(&self);
  fn print_all(&self);
} 

impl PrintAll for DataFrame {
  fn print(&self) {
     println!("{:#?}", self);
  }
  fn print_all(&self) {
    let (num_rows, num_cols) = self.shape();
    for row in 0..num_rows {
      for col in 0..num_cols {
        let value = self[col].get(row).unwrap();
        print!("{} ", value);
      }
      println!();
    }
  }
}


pub fn stdf_sumally(path:&str) -> anyhow::Result<DataFrame> {
  let mut reader = match StdfReader::new(path){
    Ok(n) => n,
    Err(e) => bail!(format!("{}",e))
  };

  let vec : Vec<_> = reader.get_record_iter()
    .map(|x| x.unwrap())
    .map(|x| x.to_string() ).collect();
  let df = df!( "value" => vec ).context("aa")?;
  let dst: DataFrame = df.groupby(["value"])?.select(["value"]).count()?;

  Ok(dst)
}

pub fn stdf_header(path:&str) -> anyhow::Result<serde_yaml::Value> {
  let mut reader = match StdfReader::new(&path){
    Ok(n) => n,
    Err(e) => bail!(format!("{}",e))
  };
  let rec_types = REC_MIR | REC_SDR | REC_WCR | REC_WIR;
  let recs : Vec<StdfRecord> = reader.get_record_iter().map(|x| x.unwrap()).filter(|x| x.is_type(rec_types)).collect();
  
  // let mut keys = Vec::<String>::new();
  // let mut value = Vec::<String>::new();
  // let mut add_rec = | n:&str, m:String | {
  //   keys.push(n.to_string());
  //   value.push(m.to_string());
  // };

  let add_val = | src:&mut serde_yaml::Value, n:String | -> anyhow::Result<()> {
    let hoge = n.lines().collect::<Vec<&str>>().into_iter()
      .skip(1).rev().skip(1).rev()
      .map(|n| n.trim().trim_end_matches(',') )
      .collect::<Vec<&str>>().join("\n");

    // let key = Value::String("MIR".to_owned());
    let val : serde_yaml::Value = serde_yaml::from_str(hoge.as_str())?;

    // if let Some(i) = src.as_mapping_mut() {
    //   i.insert(key, val);
    // } else if src.as_null().is_some() {
    //   let mut null_map = serde_yaml::Mapping::new();
    //   null_map.insert(key, val);
    //   *src = Value::Mapping(null_map);
    // } else {
    //   panic!("cannot add");
    // }
    if let Some(i) = src.as_sequence_mut() {
      i.push(val);
    } else if src.as_null().is_some() {
      let mut null_map = serde_yaml::Sequence::new();
      null_map.push(val);
      *src = Value::Sequence(null_map);
    } else {
      panic!("cannot add");
    }
    Ok(())
  };

  let mut value = serde_yaml::Value::default();
  for i in recs {
    match i {
      StdfRecord::MIR(n) => { add_val(&mut value, format!("{n:#?}"))?; },
      StdfRecord::SDR(n) => { add_val(&mut value, format!("{n:#?}"))?; },
      StdfRecord::WCR(n) => { add_val(&mut value, format!("{n:#?}"))?; }, 
      StdfRecord::WIR(n) => { add_val(&mut value, format!("{n:#?}"))?; },  
      _=> {}
    }
  }

  Ok(value)
}

pub fn stdf_ptr(path:&str, key:&str) -> anyhow::Result<DataFrame> {
  let mut cnt = -1;
  let mut vec_cnt = Vec::<i32>::new();

  let mut vec_key = Vec::<String>::new();
  let mut vec_result = Vec::<f64>::new();
  let mut vec_unit = Vec::<String>::new();
  let mut vec_real = Vec::<f64>::new();

  let mut vec_cnt2 = Vec::<i32>::new();
  let mut vec_num_test = Vec::<i32>::new();
  let mut vec_x = Vec::<i32>::new();
  let mut vec_y = Vec::<i32>::new();

  stdf!(path, |rec| {
    match rec {
      StdfRecord::MIR(n) => { println!("{:?}", n); },
      StdfRecord::SDR(n) => { println!("{:?}", n); },
      StdfRecord::PIR(n) => {
        cnt += 1;
        // println!("{:?}", n);
      },
      StdfRecord::PRR(n) => { 
        vec_cnt2.push(cnt);
        vec_num_test.push(n.num_test as i32);
        vec_x.push(n.x_coord as i32);
        vec_y.push(n.y_coord as i32);
      },
      StdfRecord::PTR(n) => { 
        if n.test_txt.contains(key) {
          vec_cnt.push(cnt);
          vec_key.push(n.test_txt);
          vec_result.push(n.result as f64);
          vec_unit.push(n.units.unwrap_or("".to_string()));
          vec_real.push(n.result as f64 * 10f64.powf(n.res_scal.unwrap_or(0i8) as f64));
        }
      },
      StdfRecord::DTR(n) => { 
        println!("{:?}",n)
      },
      _=> { }
    };
  });
  let df1 = df!(
    "cnt" => vec_cnt,
    "key" =>  vec_key,
    "result" =>  vec_result,
    "unit" => vec_unit,
    "real" =>  vec_real,
  )?;
  let df2 = df!(
    "cnt" => vec_cnt2,
    "num_test" =>  vec_num_test,
    "x" =>  vec_x,
    "y" => vec_y,
  )?;
  let joined_df = df1.left_join(&df2, ["cnt"], ["cnt"])?;

  Ok(joined_df)
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() -> anyhow::Result<()> {
    let stdf_path = "../../sample/1.stdf";
    let mut cnt = 0;
  
    let mut reader = match StdfReader::new(&stdf_path){
      Ok(n) => n,
      Err(e) => bail!(format!("{}",e))
    };
  
    for rec in reader
      .get_record_iter()
      .map(|x| x.unwrap())
    {
      cnt+=1;
      // if 247928 < cnt && cnt < 247948 {
      //   println!("{} : {:?}", cnt, rec);
      // }
      match rec {
        StdfRecord::MIR(n) => {
          println!("{} : {:?}", cnt, n);
          let dst = format!("{:#?}", n);
          println!("{}", dst);
          let result = dst.lines().collect::<Vec<&str>>().into_iter()
            .skip(1).rev().skip(1).rev()
            .map(|n| n.trim().trim_end_matches(',') )
            .collect::<Vec<&str>>().join("\n");
          let val : serde_yaml::Value = serde_yaml::from_str(result.as_str())?;
          println!("{:#?}", val);
        },
        // StdfRecord::PRR(n) => {
        //   println!("{} : {:?}", cnt, n);
        // },
        // StdfRecord::SBR(n) => {
        //   println!("{} : {:?}", cnt, n);
        // },
        _ => {}
      }
    }
    Ok(())
  }

  #[test]
  fn it_works2() -> anyhow::Result<()> {
    let stdf_path = "../../sample/1.stdf";
  
    let sumally = stdf_sumally(stdf_path)?;
    sumally.print();
    sumally.print_all();

    let header = stdf_header(stdf_path)?;
    let s = serde_yaml::to_string(&header)?;
    println!("{:#?}", header);
    println!("{}", s);

    let df = stdf_ptr(stdf_path, "Vref_VBias3_1.VBias3_pat")?;
    // let df = stdf_ptr(stdf_path, "OS_VCC.VDD12L")?;
    println!("{}", df);
    
    Ok(())
  }

}
