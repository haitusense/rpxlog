use anyhow::{Context as _};
use std::str;
// use std::fs;
use std::fs::File;
use std::io::{Read}; //, Write, BufWriter
use std::path::{PathBuf}; //Path,
use regex::Regex;
// use regex::Captures;
use polars::prelude::*;
use std::io::{BufRead, BufReader};
use indoc::{indoc, concatdoc};


/*
line毎に一度splitかけて\nに置き換えてるので\nでmatchする
*/
use once_cell::sync::Lazy;
static RE_HEADERS: Lazy<String> = Lazy::new(|| indoc!{ r"
  \[Header\]
  [\s\S]+?
  \n\n
"}.replace("\n", ""));

static RE_PIR: Lazy<String> = Lazy::new(|| indoc!{ r"
  \s*TCNT# ([0-9]+) +SITE# ([0-9]+)
  \s*NO Site  Result  TestName           Signal          Measure   LowLimit  HighLimit   Force
"}.replace("\n", ""));

static RE_PTR: Lazy<String> = Lazy::new(|| indoc!{ r"
  \s*[0-9]+ *[0-9]+ *[A-Z]+ *([^\s]+) +([^\s]+) +([\-.0-9]+ *[a-zA-Z]*?) *[\-.0-9]+[ a-zA-Z]+
"}.replace("\n", ""));

static RE_PRR: Lazy<String> = Lazy::new(|| indoc!{ r"
  \s*-------------------   ------------
  \s*Site   Fail   Total    Cate   Bin    XCoord   YCoord         TestTime\(([.0-9]+)([a-zA-Z]+)\)
  \s*-------------------   ------------
  \s*([ 0-9]+)
"}.replace("\n", ""));


enum TxtRecordEnum {
  Header(String),
  PIR(String),
  PTR(String),
  PRR(String),
  None
}

struct TxtRecord {
  header_flag: bool,
  txt: String,
  re_header : Regex,
  re_pir : Regex,
  re_ptr : Regex,
  re_prr : Regex,
}

impl TxtRecord {

  fn new() -> Self {
    Self {
      header_flag : false,
      txt : String::from(""),
      re_header : Regex::new(RE_HEADERS.replace("\n", "").as_str()).unwrap(),
      re_pir : Regex::new(RE_PIR.replace("\n", "").as_str()).unwrap(),
      re_ptr : Regex::new(RE_PTR.replace("\n", "").as_str()).unwrap(),
      re_prr : Regex::new(RE_PRR.replace("\n", "").as_str()).unwrap(),
    }
  }

  fn clear_after_header(&mut self) {
    self.txt = String::from("");
  }

  fn push(&mut self, src:&str) {
    self.txt.push_str(format!("{}\n", src).as_str());
    // println!("{}", self.txt);
  }

  fn match_type(&self) -> TxtRecordEnum {
    if self.re_ptr.is_match(self.txt.as_str()) {
      TxtRecordEnum::PTR(self.txt.to_string())
    } else if self.re_pir.is_match(self.txt.as_str()) {
      TxtRecordEnum::PIR(self.txt.to_string())
    } else if self.re_prr.is_match(self.txt.as_str()) {
      TxtRecordEnum::PRR(self.txt.to_string())
    } else if self.re_header.is_match(self.txt.as_str()) {
      TxtRecordEnum::Header(self.txt.to_string())
    } else {
      TxtRecordEnum::None
    }
  }

  fn print(&self) {
    println!("{}", self.txt);
  }

}

pub fn txt_header(path:&str) -> anyhow::Result<()> {
  let mut rec = TxtRecord::new();
  let mut line = BufReader::new(File::open(path)?).lines();

  let mut cnt = 0;
  while let Some(i) = line.next() {
    rec.push(i?.as_str());
    cnt += 1;
    match rec.match_type() {
      TxtRecordEnum::Header(n) => {
        println!("header : \n{}", n);
        rec.clear_after_header();
      },
      TxtRecordEnum::PIR(n) => {
        println!("pir : \n{}", n);
        rec.clear_after_header();
      },
      TxtRecordEnum::PTR(n) => {
        println!("ptr : \n{}", n);
        rec.clear_after_header();
      },
      TxtRecordEnum::PRR(n) => {
        println!("prr : \n{}", n);
        rec.clear_after_header();
      },
      TxtRecordEnum::None => {},
    }

        // let dst = re.captures(src.as_str()).context("caps err")?;
    // let body = dst.get(0).unwrap().as_str();
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it() -> anyhow::Result<()> {

    println!("{:?}", *RE_HEADERS);

    Ok(())
  }



  #[test]
  fn it_works3() -> anyhow::Result<()> {
    let mut src = String::new();
    let mut file = File::open(PathBuf::from("../../sample/1.txt")).context("file not found")?;
    let _ = file.read_to_string(&mut src).context("read file err")?;
    let src = src.replace("\r\n", "\n");

    let re = Regex::new(RE_HEADERS.as_str()).unwrap();
    // let re = Regex::new(RE_HEADERS).unwrap();

    println!("{}", re.is_match(src.as_str()));
    let a = re.captures(src.as_str());

    println!("{:?}", a);

    Ok(())
  }


  #[test]
  fn it_works() -> anyhow::Result<()> {
    let stdf_path = "../../sample/test.txt";
  
    let _ = txt_header(stdf_path)?;
    
    Ok(())
  }

}

