use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::sleep_ms;

use inputs::{Common, Processor};


pub struct Random {
  common: Common
}

impl Random {
    pub fn new(configuration_directive: String) -> Random {
    Random{ common: Common{configuration_directive: configuration_directive} }
  }
}

impl Processor for Random {

  fn start(&self, config: &Option<HashMap<String,String>>) -> Receiver<String> {
    let fields = vec!["fieldlist", "rate"];
    self.common.requires_fields(config, fields);
    self.common.invoke(config, Random::handle_func)
  }

  fn handle_func(tx: SyncSender<String>, config: Option<HashMap<String,String>>) {
    let rate = config.unwrap().get("rate").unwrap().clone();

    let sleep_duration: f32 = 1000.0f32 / rate.parse::<f32>().unwrap();
    println!("will sleep for {}", sleep_duration);

    loop {
      sleep_ms(300);
      let l = "foo\tbar\tqux";
      match tx.try_send(l.to_owned()) {
        Ok(()) => {},
        Err(e) => {
          println!("Unable to send line to processor: {}", e);
          println!("{}", l)
        }
      }
    }

    // for line in stdin.lock().lines() {
    //   let l = line.unwrap();
    //   let ll = l.clone();
    //   match tx.try_send(l) {
    //     Ok(()) => {},
    //     Err(e) => {
    //       println!("Unable to send line to processor: {}", e);
    //       println!("{}", ll)
    //     }
    //   }
    // }
  }
}
