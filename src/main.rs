extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;

use std::{path::Path, io::Error};
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;


#[derive(Serialize, Deserialize)]
struct HttpRequest {
    title: String,
    url: String,
    verb: String,
    content_type: String,
    data: JsonValue
}

#[derive(Serialize, Deserialize)]
struct Requests {
    requests: Vec<HttpRequest>
}

fn main() {
    let matches = clap_app!(requester =>
        (version: "0.1")
        (author: "Buddy Lindsey")
        (about: "Make common http requests you need to make")
        (@arg INPUT: +required "Command you want to run")
        (@arg TITLE: -t --title +takes_value "What do you want to run")
    ).get_matches();


    let input = matches.value_of("INPUT").unwrap();
    let title = matches.value_of("TITLE").unwrap();

    let requests: Requests = get_saved_requests().unwrap();

    match input {
        "list" => list_requests(requests),
        "execute" => process_request(requests, title.to_string()),
            &_ => println!("")
    }
}

fn list_requests(req: Requests) {
    for x in req.requests.iter() {
        println!("{}", x.title);
    }
}

fn process_request(req: Requests, title: String) {
    let request = req.requests.iter().position(|x| x.title == title).unwrap();

    make_request(request);
}

fn make_request(req: usize) {
    println!("{?}", req);
}

fn get_saved_requests() -> Result<Requests, Error> {
    let json_file_path = Path::new("data.json");
    let json_file = File::open(json_file_path).expect("File not found");
    let reader = BufReader::new(json_file);
    let deser: Requests = serde_json::from_reader(reader).expect("Error while reading json");
    Ok(deser)
}
