extern crate serde;
extern crate time;
extern crate hyper;
extern crate url;

use std::io;
use std::io::prelude::*;

use serde::json;
use serde::json::value;
use serde::json::Value;
use serde::json::ser;

use hyper::{ Client, Url};
use hyper::client::Body;
use url::form_urlencoded;



fn main() {
    let stdin = io::stdin();
    let es = "ra.ovh";

    for line in stdin.lock().lines() {
        let l = line.unwrap();

        let mut decode = json::from_str::<Value>(l.as_ref()).unwrap();

        let index_name = match decode.find("time") {
            Some(time) => time_to_index_name(time.as_string().unwrap(), None),
            None => {
                assert!(false);
                "".to_string()
            }
        };

        let typ = "logs";
        transform(&mut decode);
        let output = ser::to_string_pretty(&decode).ok().unwrap();
        println!("{}", output);

        let mut client = Client::new();
        // // /logstash-2015.05.21/logs?op_type=create
        let url = format!("http://{}:{}/{}/{}?op_type=create", es, 9200, index_name, typ );

        let uri = Url::parse(&url).ok().expect("malformed url");
        let body = output.into_bytes();
        let res = client.post(uri)
            .body(Body::BufBody(&*body, body.len()))
            .send()
            .unwrap();
        // assert_eq!(res.status, hyper::Ok);
        // let mut builder = client.post(uri);
        // builder.body(*output)
        //

    }
}

fn int_to_level(level: u64) -> String {
    match level {
        10 => "trace".to_string(),
        20 => "debug".to_string(),
        30 => "info".to_string(),
        40 => "warn".to_string(),
        50 => "error".to_string(),
        60 => "fatal".to_string(),
        _ => format!("Unknown level {}", level)
    }
}

fn transform(input: &mut Value) {
    // {"name":"stakhanov","hostname":"Quark.local","pid":65470,"level":30
    // "msg":"pushing http://fr.wikipedia.org/wiki/Giant_Sand",
    // "time":"2015-05-21T10:11:02.132Z","v":0}
    //
    // entry['@timestamp'] = entry.time;
    // entry.level = levels[entry.level];
    // entry.message = entry.msg;
    // delete entry.time;
    // delete entry.msg;
    let input = input.as_object_mut().unwrap();

    if input.contains_key("time") {
        let time = input.get("time").unwrap().clone();
        input.insert("@timestamp".to_string(), time);
        input.remove("time");
    }

    if input.contains_key("level") {
        let level = input.get("level").unwrap().as_u64().unwrap();
        input.insert("level".to_string(), value::to_value(&int_to_level(level)));
    }

    if input.contains_key("msg") {
        let message = input.get("msg").unwrap().clone();
        input.insert("message".to_string(), message);
        input.remove("msg");
    }
}

fn time_to_index_name(input: &str, option_output_format: Option<String>) -> String {
    // compatible with "2015-05-21T10:11:02.132Z"
    let format = "%Y-%m-%dT%H:%M:%S.%f%Z";

    let output_format = match option_output_format {
        Some(f) => f,
        None => "logstash-%Y.%m.%d".to_string()
    };

    match time::strptime(input, format) {
        Ok(tm) => time::strftime(output_format.as_ref(), &tm).ok().unwrap(),
        Err(e) => {
            println!("Unable to parse date:{:?}, {}.", e, input);
            let tm = time::now_utc();
            time::strftime(output_format.as_ref(), &tm).ok().unwrap()
        }
    }
}

fn read_and_transform(input: String) -> Option<Value> {
    let decode = json::from_str::<Value>(input.as_ref());

    match decode {
        Ok(val) => {
            let mut ret = val;
            transform(&mut ret);
            Some(ret)
        },
        Err(e) => {
            println!("Invalid json {:?}: {}", e, input);
            None
        }
    }
}

#[test]
fn it_transform_ok() {
    // let src = r#"{"name":"stakhanov","hostname":"Quark.local","pid":65470,"level":30,"msg":"pushing http://fr.wikipedia.org/wiki/Giant_Sand","time":"2015-05-21T10:11:02.132Z","v":0}"#;
    let src = r#"{"level":30, "msg":"this is a test.", "time": "12"}"#;
    let transformed = read_and_transform(src.to_string());
    let out = json::to_string(&transformed).unwrap();
    assert!(transformed.is_some());
    assert_eq!(out, r#"{"@timestamp":"12","level":"info","message":"this is a test."}"#);
}

#[test]
fn it_prepares_index_name() {
    // let src = r#"{"name":"stakhanov","hostname":"Quark.local","pid":65470,"level":30,"msg":"pushing http://fr.wikipedia.org/wiki/Giant_Sand","time":"2015-05-21T10:11:02.132Z","v":0}"#;
    let src = r#"{"time": "2015-05-21T10:11:02.132Z"}"#;
    let decode = json::from_str::<Value>(src).unwrap();
    match decode.find("time") {
        Some(time) => assert_eq!("logstash-2015.05.21", time_to_index_name(time.as_string().unwrap(), None)),
        None => assert!(false)
    }
}

#[test]
fn it_builds_an_es_update() {
    let src = r#"{"name":"stakhanov","hostname":"Quark.local","pid":65470,"level":30,"msg":"pushing http://fr.wikipedia.org/wiki/Giant_Sand","time":"2015-05-21T10:11:02.132Z","v":0}"#;


    let path = format!("/{}/{}", index_name, typ);
    println!("{}", path);

}
