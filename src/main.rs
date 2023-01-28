

fn main() {
    let mut path = std::env::home_dir().expect("msg");
    path.push(".oncd.json");
    let file = std::fs::File::open(path.clone()).expect("missing");
    let reader = std::io::BufReader::new(file);
    let config: serde_json::Value = serde_json::de::from_reader(reader).expect("fail");
    println!("HERE {} {} > ", path.display(), config["test"]);
}
