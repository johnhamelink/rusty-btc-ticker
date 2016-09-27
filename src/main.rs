// For Pulling down the ticker data
extern crate hyper;
use std::io::Read;
use hyper::Client;

// For deserializing the JSON into structs
extern crate rustc_serialize;
use rustc_serialize::{json, Decodable, Decoder};

// For parsing arguments
extern crate argparse;
use argparse::{ArgumentParser, Store};

struct Ticker {
    amount: String,
}

fn main() {
    // Set default arguments
    let mut currency = "GBP".to_string();

    // Handle arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("BTC Ticker");
        ap.refer(&mut currency)
            .add_option(&["-c", "--currency"],
                        Store,
                        "3-letter character code for currency (such as USD)");
        ap.parse_args_or_exit();
    }

    // Retrieve & print the result.
    println!("{}", get_price(currency));
}


impl Decodable for Ticker {
    fn decode<D: Decoder>(d: &mut D) -> Result<Ticker, D::Error> {
        d.read_struct_field("data", 2, |d| {
            let amount = try!(d.read_struct_field("amount", 0, |d| d.read_str()));
            return Ok(Ticker { amount: amount });
        })
    }
}

fn get_price(currency: String) -> (String) {
    let client = Client::new();
    let url = format!("https://api.coinbase.com/v2/prices/BTC-{}/spot", currency);

    let res = client.get(&url).send().unwrap();

    return match res.status {
        hyper::Ok => retrieve_result(res),
        _ => "".to_string(),
    };
}

fn retrieve_result(parent_res: hyper::client::response::Response) -> (String) {
    let mut body = String::new();
    let mut res = parent_res;
    res.read_to_string(&mut body).unwrap();

    match json::decode(&body) {
        Ok(Ticker { amount }) => amount.to_string(),
        Err(_) => "".to_string(),
    }
}
