//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::thread;
use std::time::Duration;
use hyper::client::Client;
use std::io::Read;

fn main() {
    loop {
        let client = Client::new();

        let mut res = client.get("http://music.abcradio.net.au/api/v1/plays/doublej/now.json").send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();

        let obj = data.as_object().unwrap();
        let now = obj.get("now").unwrap().as_object().unwrap();

        let recording = now.get("recording").unwrap().as_object().unwrap();

        let artists = recording.get("artists").unwrap().as_array().unwrap();
        let artists_iter: Vec<_> = artists.into_iter()
        .map(|x| x.as_object().unwrap().get("name").unwrap().as_string().unwrap())
        .collect();

        println!("name {:?}", artists_iter.concat());

        let title = recording.get("title").unwrap();

        println!("title {:?}", title);

        let five_seconds = Duration::new(5, 0);
        thread::sleep(five_seconds);
    }
}

// {
//     "FirstName": "John",
//     "LastName": "Doe",
//     "Age": 43,
//     "Address": {
//         "Street": "Downing Street 10",
//         "City": "London",
//         "Country": "Great Britain"
//     },
//     "PhoneNumbers": [
//         "+44 1234567",
//         "+44 2345678"
//     ]
// }

