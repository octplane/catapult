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

#[allow(dead_code)]
fn main() {
  let configuration = config::read_config_file("catapult.conf");
  match configuration  {
    Ok(conf) => {
      let ref input = conf.inputs[0];
      let data_input = match input.0.as_ref() {
        "stdin" => {
          match inputs::stdin_input(input.1.clone()) {
            Ok(data_source) => { println!("Started thread for {:?}", input.0); data_source},
            Err(e) => panic!("Unable to instanciate input stream for {:?}: {}", input.0, e)
          }
        },
        unsupported => { panic!("Input {} not implemented", unsupported)}
      };

      let ref output = conf.outputs[0];
      let data_output = match output.0.as_ref() {
        "stdout" => {
          match outputs::stdout_output(data_input, output.1.clone()) {
            Ok(data_source) => { println!("Started thread for {:?}", output.0); data_source},
            Err(e) => panic!("Unable to instanciate output stream for {:?}: {}", output.0, e)
          }
        },
        unsupported => { panic!("Output {} not implemented", unsupported)}
      };

      let _p = data_output.join();

    },
    Err(e) => panic!("{:?}", e)
  };


}
