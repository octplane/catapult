use std::io;
use std::io::prelude::*;


use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::mpsc::sync_channel;


pub fn stdin_input(_config: Option<HashMap<String,String>>) -> Result<Receiver<String>, String> {
  let (tx, rx) = sync_channel(10000);

  let reader = thread::Builder::new().name("reader".to_string()).spawn(move || {
      let stdin = io::stdin();

      for line in stdin.lock().lines() {
          let l = line.unwrap();
          let ll = l.clone();
          match tx.try_send(l) {
              Ok(()) => {},
              Err(e) => {
                  println!("Unable to send line to processor: {}", e);
                  println!("{}", ll)
              }
          }
      }
  });

  match reader {
    Ok(_) => Ok(rx),
    Err(e) => Err(format!("Unable to spawn stdin input thread: {}", e))
  }
}
