use envded::{read_yaml, parse_yaml};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
struct Sample {
    aaa: String,
    bbb: String,
}

fn main() {
    let buf = read_yaml("sample.yaml");
    let s: Sample = Sample { aaa: "".to_string(), bbb: "".to_string() };
    let res = parse_yaml(buf,s);
    println!("{:?}",res)
}
