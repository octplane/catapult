use processor::{OutputProcessor, ConfigurableFilter};

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::fs::create_dir_all;
use std::fs::OpenOptions;


use time;

/// # File output
///
/// - sends output into a rotating file
///
/// ### catapult.conf
///
/// ```
/// output {
/// 	file {
/// 		directory = "./logs/"
/// 	}
/// }
/// ```
/// ### Parameters
///
/// - **directory**: Base directory into which logs are created. Can be a strftime pattern.

pub struct RotatingFile {
  name: String
}

impl RotatingFile {
  pub fn new(name: String) -> RotatingFile {
    RotatingFile{ name: name }
  }
}

impl ConfigurableFilter for RotatingFile {
  fn human_name(&self) -> &str {
    self.name.as_str()
  }

  fn mandatory_fields(&self) -> Vec<&str> {
    vec!["directory"]
  }


}

impl OutputProcessor for RotatingFile {
  fn start(&self, rx: Receiver<String>, config: &Option<HashMap<String,String>>) -> Result<JoinHandle<()>, String> {
    self.requires_fields(config, self.mandatory_fields());
    self.invoke(rx, config, RotatingFile::handle_func)
  }
  fn handle_func(rx: Receiver<String>, oconfig: Option<HashMap<String,String>>) {
    let config = oconfig.expect("Need a configuration");
    let mut basefile_format = config.get("directory").expect("Need a log directory").clone();

    basefile_format.push_str("%Y-%m-%d-%H:00.log");

    let mut parent_dir =  PathBuf::from("/");
    let mut log_path =  PathBuf::from("");
    let mut log_file: Option<File> = None;

    loop {

      match rx.recv() {
        Ok(mut l) => {
          let now = time::now();
          let basefile = time::strftime(basefile_format.as_str(), &now).ok().unwrap();
          let new_log_path = PathBuf::from(basefile.as_str());
          let new_parent_dir = new_log_path.parent().unwrap().to_path_buf();
          if new_parent_dir != parent_dir {
            match fs::metadata(new_parent_dir.as_path()) {
              Err(_) => {create_dir_all(new_parent_dir.as_path()).ok();},
              _ => {}
            }
            parent_dir = new_parent_dir.clone();
          }

          // First time we see this file, open it, or create it
          if new_log_path != log_path {
            log_path = new_log_path.clone();
            match fs::metadata(log_path.as_path()) {
              Err(_) => {
                log_file = File::create(log_path.clone()).ok()
              },
              Ok(f) => {
                if f.is_file() {
                  log_file = OpenOptions::new().write(true).append(true).open(log_path.clone()).ok();
                } else {
                  panic!("File {:?} exists and is not a file.", log_path);
                }
              }
            }
          }
          l.push_str("\n");
          if let Some(ref mut writable) = log_file {
            let _count = writable.write_all(l.as_bytes());
          } else {
            println!("No file to write to, discarding line: {}", l);
          }
        }
        Err(e) => { panic!(e) }
      }
    }
  }
}
