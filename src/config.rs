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
enum InputKind {
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

fn keys_and_values(input:&[u8]) -> IResult<&[u8], HashMap<&str, &str> > {
  let mut h: HashMap<&str, &str> = HashMap::new();

  match keys_and_values_aggregator(input) {
    IResult::Done(i,tuple_vec) => {
      for &(k,v) in tuple_vec.iter() {
        h.insert(k, v);
      }
      IResult::Done(i, h)
    },
    IResult::Incomplete(a)     => IResult::Incomplete(a),
    IResult::Error(a)          => IResult::Error(a)
  }
}


named!(input_and_params <&[u8], (InputKind, Option<HashMap<&str,&str>>)>,
  chain!(
    blanks                     ~
    ik: input_kind                  ~
    blanks                     ~
    kv: keys_and_values?            ~
    blanks                     ,
    || { (ik, kv) }
  )
);

named!(inputs <&[u8], Vec<(InputKind, Option<HashMap<&str,&str>>)> >,
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

pub fn read_config_file(filename: &str) {
  println!("Reading config file.");
  let mut f = File::open(filename).unwrap();
  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => {
      let source = s.into_bytes();
      match inputs(&source) {
        Done(_, configuration) => println!("yes: {:?}", configuration),
        Error(e) => {
          println!("Parse error: {:?}", e);
          assert!(false);
        },
        Incomplete(e) => {
          println!("Incomplete content -> await: {:?}", e);
          assert!(false);
        }
      }
    },
    Err(e) => panic!("{:?}", e)
  };
}
