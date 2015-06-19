use std::str;

use nom::{IResult, multispace, alphanumeric, space, not_line_ending};
use nom::IResult::*;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use nom::GetInput;

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

named!(key_value    <&[u8],(&str,&str)>,
  chain!(
    key: map_res!(alphanumeric, str::from_utf8) ~
         space?                            ~
         tag!("=")                         ~
         space?                            ~
    val: map_res!(
           take_until_either!("\n;#"),
           str::from_utf8
         )                                 ~
         space?                            ~
         chain!(
           tag!("# ")        ~
           not_line_ending  ,
           ||{}
         ) ?                               ~
         multispace?                       ,
    ||{(key, val)}
  )
);


named!(keys_and_values_aggregator<&[u8], Vec<(&str,&str)> >,
 chain!(
     tag!("{")      ~
     multispace?     ~
     kva: many0!(key_value) ~
     multispace?    ~
     tag!("}"),
 || {kva} )
);

fn keys_and_values(input:&[u8]) -> IResult<&[u8], HashMap<&str, &str> > {
  let mut h: HashMap<&str, &str> = HashMap::new();
  let kva = keys_and_values_aggregator(input);

  println!("Remaining input: {:?}", kva.remaining_input());
  match kva {
    IResult::Done(i,tuple_vec) => {
        println!("{:?}", tuple_vec);
      for &(k,v) in tuple_vec.iter() {
        h.insert(k, v);
      }
      IResult::Done(i, h)
    },
    IResult::Incomplete(a)     => IResult::Incomplete(a),
    IResult::Error(a)          => {
        IResult::Error(a)
    }
  }
}


named!(input_and_params <&[u8], (InputKind, Option<HashMap<&str,&str>>)>,
  chain!(
    multispace?                     ~
    ik: input_kind                  ~
    multispace?                     ~
    kv: keys_and_values?            ~
    multispace?                     ,
    || { (ik, kv) }
  )
);

named!(inputs <&[u8], Vec<(InputKind, Option<HashMap<&str,&str>>)> >,
  chain!(
    tag!("input")                    ~
    multispace?                      ~
    tag!("{")                        ~
    multispace?                      ~
    ins: many0!(input_and_params) ~
    multispace?                      ~
    tag!("}")                        ~
    multispace?                      ,
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
          println!("parse error: {:?}", e);
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
