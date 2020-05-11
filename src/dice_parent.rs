// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct DiceChild {
    #[serde(rename = "attention")]
    pub attention: Option<String>,

    #[serde(rename = "parent")]
    pub parent: Option<DiceParent>,
}

#[derive(Serialize, Deserialize)]
pub struct DiceParent {
    #[serde(rename = "childList")]
    pub child_list: Option<Vec<DiceChild>>,

    #[serde(rename = "val1")]
    pub val1: Option<String>,

    #[serde(rename = "val2")]
    pub val2: Option<String>,
}
