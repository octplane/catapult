extern crate rand as rnd;

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::sleep_ms;
use self::rnd::{thread_rng, Rng};

use processor::{InputProcessor, ConfigurableFilter};

struct StringField;

trait Randomizable {
  fn generate(&self) -> String;
}

impl Randomizable for StringField {
  fn generate(&self) -> String {
    let s:String = thread_rng().gen_ascii_chars().take(10).collect();
    s
  }
}


pub struct Random {
  name: String
}

/// # Random input
///
/// - generate fake input according to column definitions
///
/// ### catapult.conf
///
/// ```
/// input {
///     random {
///         fieldlist = "id:id,content:str"
///         rate = 3
///     }
/// }
/// ```
/// ### Parameters
///
/// - **rate**: Number of messages per second
/// - **fieldList**: comma separated list of field name : type
///
/// ### Supported type
///
/// - String for now. All other types use string.


impl Random {
    pub fn new(name: String) -> Random {
    Random{ name: name }
  }
}

fn typeize(f: &str) -> Box<Randomizable> {
  let definition: Vec<&str> = f.split(":").collect();
  let name = definition[0];
  match definition[1] {
    _ => Box::new(StringField) as Box<Randomizable>,
  }
}

impl ConfigurableFilter for Random {
  fn human_name(&self) -> &str {
    self.name.as_ref()
  }
  fn mandatory_fields(&self) -> Vec<&str> {
    vec!["fieldlist", "rate"]
  }

}

impl InputProcessor for Random {
  fn start(&self, config: &Option<HashMap<String,String>>) -> Receiver<String> {
    self.requires_fields(config, self.mandatory_fields());
    self.invoke(config, Random::handle_func)
  }
  fn handle_func(tx: SyncSender<String>, config: Option<HashMap<String,String>>) {
    let conf = config.unwrap();
    let rate = conf.get("rate").unwrap().clone();

    let sleep_duration: u32 = (1000.0f32 / rate.parse::<f32>().unwrap()) as u32;
    println!("Random input will sleep for {}", sleep_duration);

    let fields: Vec<Box<Randomizable>> = conf.get("fieldlist").unwrap().split(",").map(move |f| typeize(f)).collect();

    loop {
      sleep_ms(sleep_duration);
      let mut l = Vec::new();
      for f in &fields {
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
