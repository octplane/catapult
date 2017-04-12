use serde_json;
use serde_json::value;
use serde_json::Value;
use serde_json::ser;

use hyper::{ Client, Url};
use hyper::client::Body;

use processor::{OutputProcessor, ConfigurableFilter};

use std;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use std::net::UdpSocket;

use filters::{transform, time_to_index_name};

pub struct Elasticsearch {
    name: String
}

impl Elasticsearch {
    pub fn new(name: String) -> Elasticsearch {
        Elasticsearch{ name: name }
    }
}

impl ConfigurableFilter for Elasticsearch {
    fn human_name(&self) -> &str {
        self.name.as_ref()
    }

    fn mandatory_fields(&self) -> Vec<&str> {
        vec!["destination", "port"]
    }
}

impl OutputProcessor for Elasticsearch {
    fn start(&self, rx:Receiver<String>, config: &Option<HashMap<String,String>>)  -> Result<JoinHandle<()>, String> {
        self.requires_fields(config, self.mandatory_fields());
        self.invoke(rx, config, Elasticsearch::handle_func)
    }
    fn handle_func(rx: Receiver<String>, oconfig: Option<HashMap<String,String>>) {
        let config = oconfig.expect("Need a configuration");

        let destination_ip = config.get("destination").expect("Need a destination IP").clone();
        let destination_port = config.get("port").expect("Need a destination port").parse::<u32>().unwrap();

        loop {
            match rx.recv() {
                    Ok(l) => {
                        match serde_json::from_str::<Value>(l.as_ref()) {
                            Ok(decoded) => {
                                let mut mutable_decoded = decoded;
                                let transformed = transform(&mut mutable_decoded);

                                println!("{:?}", transformed);

                                let index_name: Option<String> = match transformed.find("@timestamp") {
                                    Some(time) => match time.as_str() {
                                        Some(t) => Some(time_to_index_name(t)),
                                        None => {
                                            error!("Failed to stringify.");

                                            None
                                        }
                                    },
                                    None => {
                                        error!("Failed to find timestamp.");

                                        None
                                    }
                                };

                                if !index_name.is_some() {
                                    continue;
                                }

                                let index_name = index_name.unwrap();

                                let typ = "logs";
                                let output = ser::to_string(&transformed).ok().unwrap();
                                let mut client = Client::new();

                                let url = format!("http://{}:{}/{}/{}?op_type=create", destination_ip, destination_port, index_name, typ );

                                let uri = Url::parse(&url).ok().expect("malformed url");

                                debug!("Posting to elasticsearch with url: {}", url);

                                let body = output.into_bytes();

                                let res = client.post(uri)
                                    .body(Body::BufBody(&*body, body.len()))
                                    .send()
                                    .unwrap();

                                debug!("{:?}", res);
                        },
                        Err(s) => println!("Unable to parse line: {}\nfor {}",s,l)
                    }
                },
                Err(std::sync::mpsc::RecvError) => break
            }
        }
    }
}
