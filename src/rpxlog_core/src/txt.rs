use anyhow::{Context as _};
use std::str;
// use std::fs;
use std::fs::File;
use std::io::{Read}; //, Write, BufWriter
use std::path::{PathBuf}; //Path,
use regex::Regex;
// use regex::Captures;
use polars::prelude::*;


static RE_HEADERS: &str = r"
\[Header\]
[\s\S]+?
TCNT#";

static RE_HEADER: &str = r"\s*([ \S]+)=([ \S]*)\s*";

static RE_BODY: &str = r"
\s*TCNT# ([0-9]+) *SITE# ([0-9]+)
\s*([\s\S]+?)
\s*-------------------   ------------
\s*Site   Fail   Total    Cate   Bin    XCoord   YCoord         TestTime\(([.0-9]+)([a-zA-Z]+)\)
\s*-------------------   ------------
\s*([ 0-9]+)";

static RE_TEST: &str = r"
\s*[0-9]+ *[0-9]+ *[A-Z]+ *([^\s]+) +([^\s]+) +([\-.0-9]+ *[a-zA-Z]*?) *[\-.0-9]+[ a-zA-Z]+
";

// Rはバックスラッシュではなく、スラッシュ
pub fn search_dirs(path:&str, re_str:&str) -> anyhow::Result<Vec<String>> {
  let re = Regex::new(re_str).context("regex err")?;
  let dst: Vec<String> = walkdir::WalkDir::new(path).into_iter()
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.path().is_dir())
    .filter(|entry| re.is_match(&entry.path().to_string_lossy()) )
    .map(|n| n.path().to_string_lossy().to_string())
    .collect();
  Ok(dst)
}

pub fn read_logfile(path:&str) -> anyhow::Result<String> {
  let path = PathBuf::from(path);

  let mut src = String::new();
  let mut file = File::open(path).context("file not found")?;
  let _ = file.read_to_string(&mut src).context("read file err")?;

  Ok(src)
}


pub fn logheader_to_vec(src:&String) -> anyhow::Result<(Vec<&str>, Vec<&str>)> {

  let re = Regex::new(RE_HEADERS.replace("\n", "").as_str()).unwrap();
  let dst = re.captures(src.as_str()).context("caps err")?;
  let body = dst.get(0).unwrap().as_str();
  
  let re = Regex::new(RE_HEADER.replace("\n", "").as_str()).unwrap();

  let mut vec_key = Vec::<&str>::default();
  let mut vec_value = Vec::<&str>::default();

  for caps in re.captures_iter(body) {
    let key = caps.get(1).unwrap().as_str();
    let val = caps.get(2).unwrap().as_str();

    vec_key.push(key);
    vec_value.push(val);  
  }
  Ok((vec_key,vec_value))
}


#[derive(Debug)]
pub struct LogBody<'a> {
  pub tcnt : Vec<i32>,
  pub site : Vec<i32>,
  pub test_index : Vec<i32>,
  pub key : Vec<&'a str>,
  pub signal : Vec<&'a str>,
  pub value : Vec<&'a str>,
}

impl<'a> LogBody<'a> {
  fn new() -> Self {
    Self {
      tcnt : Vec::<i32>::default(),
      site : Vec::<i32>::default(),
      test_index: Vec::<i32>::default(),
      key : Vec::<&str>::default(),
      signal : Vec::<&str>::default(),
      value : Vec::<&str>::default()
    }
  }

  fn add(&mut self, tcnt:i32, site:i32, index:i32, key:&'a str, signal:&'a str, value:&'a str) {
    self.tcnt.push(tcnt);
    self.site.push(site);
    self.test_index.push(index);
    self.key.push(key);
    self.signal.push(signal);
    self.value.push(value);
  }
}

pub fn logbody_to_vec(src:&String) -> anyhow::Result<LogBody> {

  let mut dst = LogBody::new();

  let re = Regex::new(RE_BODY.replace("\n", "").as_str()).unwrap();
  for caps in re.captures_iter(src.as_str()) {
    let tcnt = caps.get(1).unwrap().as_str().parse().unwrap();
    let site = caps.get(2).unwrap().as_str().parse().unwrap();
    let body = caps.get(3).unwrap().as_str();
    let testtime = caps.get(4).unwrap().as_str();
    let t_unit = caps.get(5).unwrap().as_str();
    let result = caps.get(6).unwrap().as_str();

    dst.add(tcnt, site, 0, "testtime", "", testtime);
    dst.add(tcnt, site, 0, "t_unit", "",t_unit);

    let buf = result.split_whitespace().collect::<Vec<_>>();
    dst.add(tcnt, site, 0, "Site", "",*buf.get(0).unwrap_or_else(|| &""));
    dst.add(tcnt, site, 0, "Fail", "",*buf.get(1).unwrap_or_else(|| &""));
    dst.add(tcnt, site, 0, "Total", "",*buf.get(2).unwrap_or_else(|| &""));
    dst.add(tcnt, site, 0, "Cate", "",*buf.get(3).unwrap_or_else(|| &""));
    dst.add(tcnt, site, 0, "Bin", "",*buf.get(4).unwrap_or_else(|| &""));
    dst.add(tcnt, site, 0, "XCoord", "",*buf.get(5).unwrap_or_else(|| &""));
    dst.add(tcnt, site, 0, "YCoord", "",*buf.get(6).unwrap_or_else(|| &""));

    let mut index = 0i32; 
    let re = Regex::new(RE_TEST.replace("\n", "").as_str()).unwrap();
    for caps2 in re.captures_iter(body) {
      index += 1;
      dst.add(tcnt, site, index,
        caps2.get(1).unwrap().as_str(),
        caps2.get(2).unwrap().as_str(),
        caps2.get(3).unwrap().as_str());
    }
  }

  Ok(dst)
}


pub fn txt_ptr_to_robj(path:&str, key:&str) -> anyhow::Result<DataFrame> {
  let mut src = String::new();
  let mut file = File::open(PathBuf::from(path)).context("file not found")?;
  let _ = file.read_to_string(&mut src).context("read file err")?;

  let vec = logbody_to_vec(&src)?;
  
  let mut vec_cnt = Vec::<i32>::new();
  let mut vec_key = Vec::<String>::new();
  let mut vec_result = Vec::<f64>::new();
  let mut vec_unit = Vec::<String>::new();
  let mut vec_real = Vec::<f64>::new();

  for i in 0..vec.tcnt.len() {
    let a = vec.key[i];
    let b = vec.signal[i];
    if format!("{a}.{b}") == key {

      let hoge = super::unit::value_with_unit(vec.value[i])?;
      vec_cnt.push(vec.tcnt[i]);
      vec_key.push(key.to_string());
      vec_result.push(hoge.0);
      vec_unit.push(hoge.1);
      vec_real.push(hoge.2);
      // println!("{} : {} = {}", vec.tcnt[i], key, vec.value[i]);
    }
  }

  let df = df!(
    "cnt" => vec_cnt,
    "key" =>  vec_key,
    "result" =>  vec_result,
    "unit" => vec_unit,
    "real" =>  vec_real,
  )?;
  Ok(df)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works1() -> anyhow::Result<()> {
    let src = read_logfile("../dummy.txt")?;

    let df = logheader_to_vec(&src)?;

    println!("{df:?}");

    Ok(())
  }

  #[test]
  fn it_works2() -> anyhow::Result<()> {
    let src = read_logfile("../dummy.txt")?;

    let df = logbody_to_vec(&src)?;

    println!("{df:?}");

    Ok(())
  }

  #[test]
  fn it_works3() -> anyhow::Result<()> {
    let src = "   0      1       2       1     1       5       1";

    let a = src.split_whitespace().collect::<Vec<_>>();
    println!("{a:?}");
    let _b = a[0];
    let _c = a[1];
    let d = *a.get(8).unwrap_or_else(|| &"no");
    println!("{d:?}");

    let v: Vec<&str> = src.matches(r"1").collect();
    println!("{v:?}");

    let s = String::from("rust string sample program rust i like rust");
    let v: Vec<_> = s.match_indices("rust").collect();
    println!("{v:?}");

    Ok(())
  }

  #[test]
  fn it_works4() -> anyhow::Result<()> {
    let i = 10usize;
    let v2 = vec![0i32; i]
      .into_iter().enumerate()
      .map(|(i, _n)| i as i32)
      .collect::<Vec<_>>();
    println!("{v2:?}");
    Ok(())
  }
  
  use polars::prelude::*;

  #[test]
  fn it_works_df2() -> anyhow::Result<()> {
    let src = read_logfile("../dummy.txt")?;
    let df = logheader_to_vec(&src)?;

    let mut _df1: DataFrame = df!(
      "key" => &df.0,
      "value" => &df.1,
    ).unwrap();
   


  //  let mut a = *(&df1["key"]);
  //  let b = a.into_iter()
  //  .map(|v| v);

    println!("{_df1:?}");

    Ok(())
  }

}