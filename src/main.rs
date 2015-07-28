#![feature(convert)]
#![feature(vec_push_all)]

#[macro_use]
extern crate nom;

extern crate serde;
extern crate chrono;
extern crate hyper;
extern crate url;

pub mod config;
pub mod inputs;
pub mod outputs;
pub mod filters;
pub mod processor;

use processor::{InputProcessor, OutputProcessor};

#[allow(dead_code)]
fn main() {
  let configuration = config::read_config_file("catapult.conf");
  match configuration  {
    Ok(conf) => {
      let ref input = conf.inputs[0];
      let ref datasource_name = input.0;
      let ref args = conf.inputs[0].1;
      let data_input = match datasource_name.as_str() {
        "stdin" => inputs::stdin::Stdin::new(datasource_name.to_owned()).start(args),
        "random" => inputs::random::Random::new(datasource_name.to_owned()).start(args),
        unsupported => { panic!("Input {} not implemented", unsupported)}
      };

      let ref output = conf.outputs[0];
      let ref dataoutput_name = output.0;
      let ref oargs = output.1;
      let data_output = match output.0.as_ref() {
        "stdout" => outputs::stdout::Stdout::new(dataoutput_name.to_owned()).start(data_input, oargs),
        // "network" => outputs::network::Network::new(data_input).start(output.1.clone()),
        unsupported => { panic!("Output {} not implemented", unsupported)}
      };

      let _p = data_output.unwrap().join();

    },
    Err(e) => panic!("{:?}", e)
  };


}
