use std::process::Command;
use std::str;
use serde_json::{self};

fn main() {
  let snowfl = SnowFl{};
  let search_result_json = snowfl.get("ubuntu");
  println!("{:?}", search_result_json.to_string());
}

struct SnowFl;
trait WebApi {
  fn new() -> Self;
  fn get(&self, query: &str) -> serde_json::Value;
  fn make_request(&self, query: String) -> Vec<u8>;
  fn make_request_to_json(&self, curl_make_o: Vec<u8>) -> serde_json::Result<serde_json::Value>;
}

impl WebApi for SnowFl {
  fn new() -> SnowFl {
    SnowFl {}
  }
  fn get(&self, query: &str) -> serde_json::Value {
    let result = self.make_request_to_json(self.make_request(query.to_string()));
    let value: serde_json::Value = result.into_iter().collect();
    return value;
  }
  fn make_request(&self, query: String) -> Vec<u8> {
    let mut curl = Command::new("sh");
    curl.arg("-c")
              .arg(
                "curl 'https://snowfl.com/pjoJhSpcOzhjpFoOCzcpruhtSOBFAPuKIznmehc/".to_owned() + &query + "/l9JIMrAo/0/NONE/NONE/1?_=1622732709897' \
                -H 'authority: snowfl.com' \
                -H 'sec-ch-ua: \" Not;A Brand\";v=\"99\", \"Google Chrome\";v=\"91\", \"Chromium\";v=\"91\"' \
                -H 'accept: application/json, text/javascript, */*; q=0.01' \
                -H 'x-requested-with: XMLHttpRequest' \
                -H 'sec-ch-ua-mobile: ?0' \
                -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36' \
                -H 'sec-fetch-site: same-origin' \
                -H 'sec-fetch-mode: cors' \
                -H 'sec-fetch-dest: empty' \
                -H 'referer: https://snowfl.com/' \
                -H 'accept-language: en-GB,en-US;q=0.9,en;q=0.8,pl;q=0.7' \
                --compressed"
              );
    let output = curl.output().expect("failed to execute process");
    assert!(output.status.success());
    return output.stdout;
  }
  fn make_request_to_json(&self, curl_make_o: Vec<u8>) -> serde_json::Result<serde_json::Value> {
    let s = match str::from_utf8(&curl_make_o) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let json: serde_json::Result<serde_json::Value> = serde_json::from_str(s);
    return json;
  }
}

