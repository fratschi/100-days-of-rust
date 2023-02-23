use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub userid: i64,
    pub name: String,
    #[serde(rename = "given_name")]
    pub given_name: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_ok() {
        let data = r#"{
            "userid": 1234,
            "name": "Doe",
            "given_name": "John"
        }"#;

        let person: Person = serde_json::from_str(data).expect("error");
        println!("{:?}", person)
    }
}
