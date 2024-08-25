
use serde::{ Serialize, Deserialize};
use serde_json::from_str;


#[derive(Serialize, Deserialize, Debug)]
struct ActorDetails {
    actor: String,
    film: String,
    rating: u32,
}

fn main() {
    let json = std::fs::read_to_string("vid.json").unwrap();
    let result = from_str::<Vec<ActorDetails>>(&json);
    println!("{:#?}", result);
}
