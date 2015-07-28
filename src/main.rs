#![feature(convert)]
#![feature(vec_push_all)]

#[macro_use]
extern crate nom;

extern crate serde;
extern crate chrono;
extern crate hyper;
extern crate url;
extern crate time;

extern crate docopt;

pub mod config;
pub mod inputs;
pub mod outputs;
pub mod filters;
pub mod processor;

use docopt::Docopt;
use processor::{InputProcessor, OutputProcessor};

// Write the Docopt usage string. dfrites ?
static USAGE: &'static str = "
Usage: catapult [-c CONFIGFILE]
       catapult (--help | -h)

Options:
    -h, --help     Show this screen.
    -c CONFIGFILE  Configuration file [default: catapult.conf]
";

#[allow(dead_code)]
fn main() {
  // Parse argv and exit the program with an error message if it fails.
  let args = Docopt::new(USAGE)
    .and_then(|d| d.argv(std::env::args().into_iter()).parse())
    .unwrap_or_else(|e| e.exit());

  let config_file = args.get_str("-c");

  let configuration = config::read_config_file(config_file);
  match configuration  {
    Ok(conf) => {
      let ref input = conf.inputs[0];
      let ref datasource_name = input.0;
      let ref args = conf.inputs[0].1;
      let data_input = match datasource_name.as_str() {
        "stdin" => inputs::stdin::Stdin::new(datasource_name.to_owned()).start(args),
        "random" => inputs::random::Random::new(datasource_name.to_owned()).start(args),
        "network" => inputs::network::Network::new(datasource_name.to_owned()).start(args),
        unsupported => { panic!("Input {} not implemented", unsupported)}
      };

      let ref output = conf.outputs[0];
      let ref dataoutput_name = output.0;
      let ref oargs = output.1;
      let data_output = match output.0.as_ref() {
        "stdout" => outputs::stdout::Stdout::new(dataoutput_name.to_owned()).start(data_input, oargs),
        "network" => outputs::network::Network::new(dataoutput_name.to_owned()).start(data_input, oargs),
        "file" => outputs::file::RotatingFile::new(dataoutput_name.to_owned()).start(data_input, oargs),
        unsupported => { panic!("Output {} not implemented", unsupported)}
      };

      let _p = data_output.unwrap().join();

    },
    Err(e) => panic!("{:?}", e)
  };


}
