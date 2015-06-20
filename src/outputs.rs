use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;


pub fn stdout_output(rx: Receiver<String>, _config: Option<HashMap<String,String>>) -> Result<JoinHandle<()>, String> {
  let processor = thread::Builder::new().name("processor".to_string()).spawn(move ||{
    loop {
      match rx.recv() {
        Ok(l) => { println!("{}", l) }
        Err(e) => { panic!(e) }
      }
    }
  });

  match processor {
    Ok(p) => Ok(p),
    Err(e) => Err(format!("Unable to spawn stdout output thread: {}", e))
  }
}

// use serde::json;
// use serde::json::value;
// use serde::json::Value;
// use serde::json::ser;

// use hyper::{ Client, Url};
// use hyper::client::Body;


  // let processor = thread::Builder::new().name("processor".to_string()).spawn(move ||{
  //   loop {
  //     match rx.recv() {
  //         Ok(l) => {
  //           println!("read: {}", l);
  //           match json::from_str::<Value>(l.as_ref()) {
  //             Ok(decoded) => {
  //               let mut mutable_decoded = decoded;
  //               let transformed = transform(&mut mutable_decoded);
  //
  //               println!("{:?}", transformed);
  //
  //               let index_name = match transformed.find("@timestamp") {
  //                 Some(time) => match time.as_string() {
  //                   Some(t) => time_to_index_name(t),
  //                   None => {
  //                     println!("Unable to stringify {:?}", time);
  //                     assert!(false);
  //                     "".to_string()
  //                   }
  //                 },
  //                 None => {
  //                   assert!(false);
  //                   "".to_string()
  //                 }
  //               };
  //
  //               let typ = "logs";
  //
  //               let output = ser::to_string(&transformed).ok().unwrap();
  //               let mut client = Client::new();
  //               // // /logstash-2015.05.21/logs?op_type=create
  //               let url = format!("http://{}:{}/{}/{}?op_type=create", es, 9200, index_name, typ );
  //
  //               let uri = Url::parse(&url).ok().expect("malformed url");
  //               let body = output.into_bytes();
  //               let _ = client.post(uri)
  //                 .body(Body::BufBody(&*body, body.len()))
  //                 .send()
  //                 .unwrap();
  //           },
  //           Err(s) => println!("Unable to parse line: {}\nfor {}",s,l)
  //         }
  //       },
  //       Err(std::sync::mpsc::RecvError) => break
  //     }
  //
  //   }
  // }).ok().expect("Unable to unwrap thread for processor");
