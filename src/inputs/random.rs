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
  fn human_name(&self) -> &str {
    self.common.human_name()
  }
  fn mandatory_fields(&self) -> Vec<&str> {
    vec!["fieldlist", "rate"]
  }

  fn start(&self, config: &Option<HashMap<String,String>>) -> Receiver<String> {
    self.common.requires_fields(config, self.mandatory_fields());
    self.common.invoke(config, Random::handle_func)
  }

  fn handle_func(tx: SyncSender<String>, config: Option<HashMap<String,String>>) {
    let rate = config.unwrap().get("rate").unwrap().clone();

    let sleep_duration: u32 = (1000.0f32 / rate.parse::<f32>().unwrap()) as u32;
    println!("will sleep for {}", sleep_duration);

    loop {
      sleep_ms(sleep_duration);
      let l = "foo\tbar\tqux";
      match tx.try_send(l.to_owned()) {
        Ok(()) => {},
        Err(e) => {
          println!("Unable to send line to processor: {}", e);
          println!("{}", l)
        }
      }
    }
  }
}
