use std::str;

use nom::{Consumer,ConsumerState, MemProducer, FileProducer, multispace};
use nom::IResult::*;


use nom::ConsumerState::{ConsumerError, Await, Seek};
use std::io::SeekFrom;

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
  File{fname: String},
  None,
}

named!(file_and_name <InputKind>,
  chain!(
    tag!("file")             ~
    multispace              ~
    fname: quoted_string    ,
    || { InputKind::File{fname:fname.to_string()} }
  )
);

named!(stdin <InputKind>,
  chain!(
    tag!("stdin"),
    || { println!("stdin!");
      InputKind::Stdin}
  )
);

named!(input_configurer <InputKind>, alt!(stdin | file_and_name));

#[allow(dead_code)]
pub struct ConfigFileConsumer {
  input: InputKind
}

impl Consumer for ConfigFileConsumer {
  fn consume(&mut self, input: &[u8]) -> ConsumerState {
    println!("consuming input: {:?}", input);
    match input_configurer(input) {
      Done(_, _) => return ConsumerState::ConsumerDone,
      Error(a) => {
        println!("parse error: {:?}", a);
        assert!(false);
        return ConsumerState::ConsumerError(0);
      },
      Incomplete(_) => {
        println!("Incomplete content -> await: {}", input.len());
        return ConsumerState::Await(0, input.len());
      }
    }
  }
  fn failed(&mut self, error_code: u32) {
    println!("failed with error code: {}", error_code);
   }

  fn end(&mut self) {
    println!("finished!");
  }

  //
  // fn run(&mut self, producer: &mut Producer) {
  //   let mut acc: Vec<u8>       = Vec::new();
  //   let mut position           = 0;
  //   let mut should_seek        = false;
  //   let mut consumed:usize     = 0;
  //   let mut needed:usize       = 0;
  //   let mut seek_from:SeekFrom = SeekFrom::Current(0);
  //   let mut eof = false;
  //   let mut end = false;
  //   let mut err: Option<u32> = None;
  //
  //   loop {
  //     //self.getDataFromProducer(producer, seek_from, needed, acc);
  //     if !should_seek && acc.len() - consumed >= needed {
  //       //println!("buffer is large enough, skipping");
  //       let mut tmp = Vec::new();
  //       //println!("before:\n{}", acc.to_hex(16));
  //       //println!("after:\n{}", (&acc[consumed..acc.len()]).to_hex(16));
  //       tmp.extend(acc[consumed..acc.len()].iter().cloned());
  //       acc.clear();
  //       acc = tmp;
  //     } else {
  //       if should_seek {
  //         let _ = producer.seek(seek_from);
  //         //println!("seeking: {:?}", pos);
  //         should_seek = false;
  //         acc.clear();
  //       } else {
  //         let mut tmp = Vec::new();
  //         tmp.extend(acc[consumed..acc.len()].iter().cloned());
  //         acc.clear();
  //         acc = tmp;
  //       }
  //
  //       loop {
  //         let state   = producer.produce();
  //         match state {
  //           Data(v) => {
  //             //println!("got data: {} bytes", v.len());
  //             acc.extend(v.iter().cloned());
  //             position = position + v.len();
  //           },
  //           Eof(v) => {
  //             if v.is_empty() {
  //               //println!("eof empty");
  //               //eof = true;
  //               self.end();
  //               return
  //             } else {
  //               //println!("eof with {} bytes", v.len());
  //               eof = true;
  //               acc.extend(v.iter().cloned());
  //               position = position + v.len();
  //               break;
  //             }
  //           }
  //           ProducerError(_) => {break;}
  //           Continue => { continue; }
  //         }
  //         //println!("acc size: {}", acc.len());
  //         if acc.len() >= needed { break; }
  //       }
  //     }
  //
  //     //println!("full:\n{}", acc.to_hex(8));
  //     //println!("truncated:\n{}", (&acc[0..needed]).to_hex(16));
  //     match self.consume(&acc[0..needed]) {
  //       ConsumerState::ConsumerError(e) => {
  //         //println!("consumer error, stopping: {}", e);
  //         err = Some(e);
  //       },
  //       ConsumerState::ConsumerDone => {
  //         //println!("data, done");
  //         end = true;
  //       },
  //       ConsumerState::Seek(consumed_bytes, sf, needed_bytes) => {
  //         //println!("Seek: consumed {} bytes, got {:?} and asked {} bytes", consumed_bytes, sf, needed_bytes);
  //         seek_from = match sf {
  //           SeekFrom::Current(i) => SeekFrom::Current(i - (acc.len() - needed) as i64),
  //           a => a
  //         };
  //         should_seek = true;
  //         consumed = consumed_bytes;
  //         needed   = needed_bytes;
  //       },
  //       ConsumerState::Await(consumed_bytes, needed_bytes) => {
  //         //println!("consumed: {} bytes | needed: {} bytes", consumed_bytes, needed_bytes);
  //         consumed = consumed_bytes;
  //         needed   = needed_bytes;
  //       },
  //       ConsumerState::Incomplete => {
  //         //println!("incomplete");
  //       }
  //     }
  //     if let Some(e) = err {
  //       self.failed(e);
  //       break;
  //     }
  //     if eof {
  //       self.end();
  //       break;
  //     }
  //     if end {
  //       self.end();
  //       break;
  //     }
  //   }
  // }
  //


}

pub fn load_config() {
  println!("## Hand made consuming");
  let source = String::from("stdin").into_bytes();
  let mut c = ConfigFileConsumer{input: InputKind::None};
  println!("{:?}", c.consume(&source));
}


pub fn read_mem() {
  println!("## Memory producer");

  let mut p = MemProducer::new(&b"stdin"[..], 3);
  let mut c = ConfigFileConsumer{input: InputKind::None};
  c.run(&mut p);
}

#[allow(unused_must_use)]
pub fn read_config_file(filename: &str) {
  println!("## Reading config file");
  FileProducer::new(filename, 400).map(|producer: FileProducer| {
    println!("Reading configuration file in {}", filename);
    let mut p = producer;
    let mut c = ConfigFileConsumer{input: InputKind::None};
    c.run(&mut p);
  });
}
