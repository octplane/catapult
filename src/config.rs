use std::str;

use nom::{IResult, multispace, eof, alphanumeric, space, not_line_ending};
use nom::IResult::*;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

named!(quoted_string <&str>,
  chain!(
    tag!("\"")              ~
    qs: map_res!(
      take_until!("\""),
      str::from_utf8)     ~
    tag!("\"")              ,
  || { qs }
  )
);

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InputKind {
  Stdin,
  File,
  None,
}

named!( file <InputKind>, chain!( tag!("file"), || { InputKind::File } ) );
named!( stdin <InputKind>, chain!( tag!("stdin"), || { InputKind::Stdin } ) );
named!( none <InputKind>, chain!( tag!("none"), || { InputKind::None } ) );
named!(input_kind <InputKind>, alt!(stdin | file | none));

named!(blanks,
    chain!(
        many0!(alt!(multispace | comment | eol | eof)),
        || { &b""[..] }));

named!(comment,
    chain!(
        tag!("#") ~
        not_line_ending? ~
        alt!(eol | eof),
        || { &b""[..] }));

named!(eol,
    alt!(tag!("\r\n") | tag!("\n") | tag!("\u{2028}") | tag!("\u{2029}")));


named!(key_value    <&[u8],(&str,&str)>,
  chain!(
    key: map_res!(alphanumeric, str::from_utf8) ~
      space?                            ~
      tag!("=")                         ~
      space?                            ~
    val: alt!(
      quoted_string |
      map_res!(
        take_until_either!("\n\r#"),
        str::from_utf8
      )
      )                                    ~
      blanks                               ,
    ||{(key, val)}
  )
);


named!(keys_and_values_aggregator<&[u8], Vec<(&str,&str)> >,
 chain!(
     tag!("{")      ~
     blanks     ~
     kva: many0!(key_value) ~
     blanks    ~
     tag!("}"),
 || {kva} )
);

fn keys_and_values(input:&[u8]) -> IResult<&[u8], HashMap<String, String> > {
  let mut h: HashMap<String, String> = HashMap::new();

  match keys_and_values_aggregator(input) {
    IResult::Done(i,tuple_vec) => {
      for &(k,v) in tuple_vec.iter() {
        h.insert(k.to_owned(), v.to_owned());
      }
      IResult::Done(i, h)
    },
    IResult::Incomplete(a)     => IResult::Incomplete(a),
    IResult::Error(a)          => IResult::Error(a)
  }
}


named!(input_and_params <&[u8], (InputKind, Option<HashMap<String,String>>)>,
  chain!(
    blanks                     ~
    ik: input_kind                  ~
    blanks                     ~
    kv: keys_and_values?            ~
    blanks                     ,
    || { (ik, kv) }
  )
);

named!(inputs <&[u8], Vec<(InputKind, Option<HashMap<String,String>>)> >,
  chain!(
    tag!("input")                    ~
    blanks                      ~
    tag!("{")                        ~
    blanks                      ~
    ins: many0!(input_and_params) ~
    blanks                      ~
    tag!("}")                        ~
    blanks                      ,
    || { (ins) }
  )
);

pub fn read_config_file(filename: &str) -> Result<Vec<(InputKind,  Option<HashMap<String,String>>)>, String> {
  println!("Reading config file.");
  let mut f = File::open(filename).unwrap();
  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => {
      let source = s.into_bytes();
      match inputs(&source) {
        Done(_, configuration) => Ok(configuration),
        Error(e) => {
          Err(format!("Parse error: {:?}", e))
        },
        Incomplete(e) => {
          Err(format!("Incomplete content -> await: {:?}", e))
        }
      }
    },
    Err(e) => Err(format!("Read error: {:?}", e))
  }
}

#[test]
fn test_config_parser() {
  match read_config_file("files/test_config.conf") {
    Ok(conf) => {
      // Some({"path": "some literal string", "pipo": "12"})), (Stdin, Some({"tag": "stdin"}))]
      assert_eq!(conf.len(), 2);
      assert_eq!(conf[0].0, InputKind::File);
      let mut file_conf = HashMap::new();
      file_conf.insert("path".to_owned(), "some literal string".to_owned());
      file_conf.insert("pipo".to_owned(), "12".to_owned());
      assert_eq!(conf[0].1, Some(file_conf) );

      assert_eq!(conf[1].0, InputKind::Stdin);
      let mut stdin_conf = HashMap::new();
      stdin_conf.insert("tag".to_owned(), "stdin".to_owned());
      assert_eq!(conf[1].1, Some(stdin_conf) );


    },
    Err(e) => assert!(false, format!("Unable to parse configuration file: {}", e))
  }
}
