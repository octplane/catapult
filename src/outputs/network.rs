use processor::{OutputProcessor, ConfigurableFilter};

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use std::net::UdpSocket;

pub struct Network {
  name: String
}

impl Network {
  pub fn new(name: String) -> Network {
    Network{ name: name }
  }
}

impl ConfigurableFilter for Network {
  fn human_name(&self) -> &str {
    self.name.as_str()
  }

}

impl OutputProcessor for Network {
  fn start(&self, rx:Receiver<String>, config: &Option<HashMap<String,String>>)  -> Result<JoinHandle<()>, String> {
    self.invoke(rx, config, Network::handle_func)
  }
  fn handle_func(rx: Receiver<String>, oconfig: Option<HashMap<String,String>>) {
    let config = oconfig.expect("Need a configuration");

    let destination_ip = config.get("destination").expect("Need a destination IP").clone();
    let destination_port = config.get("port").expect("Need a destination port").parse::<u32>().unwrap();

    let udp = UdpSocket::bind("0.0.0.0:0").unwrap();
    let dest = format!("{}:{}", destination_ip, destination_port);

    loop {
      match rx.recv() {
        Ok(l) => {
          udp.send_to(l.as_bytes(), dest.as_str()).unwrap();
        },
        Err(e) => { panic!(e) }
      }
    }
  }
}
