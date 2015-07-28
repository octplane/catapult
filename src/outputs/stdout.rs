use processor::{OutputProcessor, ConfigurableFilter};

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

/// # Stdout output
///
/// - sends output to stdout
///
/// ### catapult.conf
///
/// ```
/// output {
///   stdout
/// }
/// ```
/// ### Parameters
///
/// - none


pub struct Stdout {
  name: String
}

impl Stdout {
  pub fn new(name: String) -> Stdout {
    Stdout{ name: name }
  }
}

impl ConfigurableFilter for Stdout {
  fn human_name(&self) -> &str {
    self.name.as_str()
  }

}

impl OutputProcessor for Stdout {
  fn start(&self, rx: Receiver<String>, config: &Option<HashMap<String,String>>) -> Result<JoinHandle<()>, String> {
    self.invoke(rx, config, Stdout::handle_func)
  }
  fn handle_func(rx: Receiver<String>, _config: Option<HashMap<String,String>>) {
      loop {
        match rx.recv() {
          Ok(l) => { println!("{}", l) }
          Err(e) => { panic!(e) }
        }
      }
  }
}
