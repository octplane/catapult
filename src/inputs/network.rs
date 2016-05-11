use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::net::UdpSocket;

use processor::{InputProcessor, ConfigurableFilter};
/// # Network input
///
/// - listens on an UDP port for data, forward upstream
///
/// ### catapult.conf
///
/// ```
/// input {
///   network {
///     listenPort = 6667
///   }
/// }
/// ```
/// ### Parameters
///
/// - **listenPort**: UDP port to listen to


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
    self.name.as_ref()
  }

  fn mandatory_fields(&self) -> Vec<&str> {
    vec!["listenPort"]
  }

}

impl InputProcessor for Network {
  fn start(&self, config: &Option<HashMap<String,String>>) -> Receiver<String> {
    self.requires_fields(config, self.mandatory_fields());
    self.invoke(config, Network::handle_func)
  }
  fn handle_func(tx: SyncSender<String>, oconfig: Option<HashMap<String,String>>) {
    let config = oconfig.expect("Need a configuration");
    let listen_port = config.get("listenPort").expect("Need a listen port").parse::<u32>().unwrap();

    let udp = UdpSocket::bind(&*format!("0.0.0.0:{}", listen_port)).unwrap();

    loop {
      let mut buf = [0; 1024];
      let _ = udp.recv_from(&mut buf).unwrap();
      let l = String::from_utf8_lossy(&buf).into_owned();
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
