use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::sleep_ms;

use processor::{Common, Processor};

#[derive(Clone)]
enum Kind {
  String
}

#[derive(Clone)]
struct GeneratedType{
  name: String,
  kind: Kind,
}

impl GeneratedType {
  pub fn generate(&self) -> String {
    "foo".to_string()
  }
}

pub struct Random {
  common: Common
}

impl Random {
    pub fn new(configuration_directive: String) -> Random {
    Random{ common: Common{configuration_directive: configuration_directive} }
  }
}

fn typeize(f: &str) -> GeneratedType {
  let definition: Vec<&str> = f.split(":").collect();
  let name = definition[0];
  match definition[1] {
    _ => GeneratedType{name:name.to_string(), kind: Kind::String }
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
    let conf = config.unwrap();
    let rate = conf.get("rate").unwrap().clone();

    let sleep_duration: u32 = (1000.0f32 / rate.parse::<f32>().unwrap()) as u32;
    println!("Random input will sleep for {}", sleep_duration);

    let fields: Vec<GeneratedType> = conf.get("fieldlist").unwrap().split(",").map(move |f| typeize(f)).collect();


    loop {
      sleep_ms(sleep_duration);
      let mut l = Vec::new();
      for f in fields.clone() {
        l.push(f.generate());
      }
      let line = l.connect("\t");
      match tx.try_send(line.clone()) {
        Ok(()) => {},
        Err(e) => {
          println!("Unable to send line to processor: {}", e);
          println!("{}", line)
        }
      }
    }
  }
}
