use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};

use processor::{Common,Processor};

pub struct Stdin {
  common: Common
}

impl Stdin {
    pub fn new(configuration_directive: String) -> Stdin {
    Stdin{ common: Common{configuration_directive: configuration_directive} }
  }
}

impl Processor for Stdin {
  fn human_name(&self) -> &str {
    self.common.human_name()
  }

  fn start(&self, config: &Option<HashMap<String,String>>) -> Receiver<String> {
    self.common.invoke(config, Stdin::handle_func)
  }

  fn handle_func(tx: SyncSender<String>, _config: Option<HashMap<String,String>>) {
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
  }
}
