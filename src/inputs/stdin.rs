use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};

use processor::{InputProcessor, ConfigurableFilter};

/// # Stdin input
///
/// - reads stdin
///
/// ### catapult.conf
///
/// ```
/// input {
///   stdin
/// }
/// ```
/// ### Parameters
///
/// - none

pub struct Stdin {
  name: String
}

impl Stdin {
  pub fn new(name: String) -> Stdin {
    Stdin{ name: name }
  }
}

impl ConfigurableFilter for Stdin {
  fn human_name(&self) -> &str {
    self.name.as_str()
  }
}

impl InputProcessor for Stdin {
  fn start(&self, config: &Option<HashMap<String,String>>) -> Receiver<String> {
    self.invoke(config, Stdin::handle_func)
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
