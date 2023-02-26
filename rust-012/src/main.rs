use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
enum Item {
    Single { name: String, sla: f32 },
    Serial { items: Vec<Item> },
    Parallel { items: Vec<Item> },
}

fn sla(sla: f32) -> f32 {
    sla / 100f32
}

fn calculate_sla(items: &Item) -> f32 {
    let mut sla = 1f32;

    match items {
        Item::Single { sla: s, .. } => {
            sla *= s;
        }
        Item::Serial { items: i } => {
            let mut inner = 1f32;
            for item in i {
                inner *= calculate_sla(item);
            }
            sla *= inner;
        }
        Item::Parallel { items: i } => {
            let mut inner = 1f32;
            for item in i {
                inner *= 1f32 - calculate_sla(item);
            }
            sla *= 1f32 - inner;
        }
    }

    return sla;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let contents = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    let sladata = serde_json::from_str(&contents).unwrap();

    let sla = calculate_sla(&sladata)*100f32;

    println!("Overall SLA: {}%", sla);


}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple() {
        let s = Item::Single {
            name: "A".to_string(),
            sla: sla(99.9),
        };
        let j = serde_json::to_string(&s).unwrap();

        println!("{:?}", j);

        let l = Item::Serial {
            items: vec![
                Item::Single {
                    name: "A".to_string(),
                    sla: sla(99.9),
                },
                Item::Single {
                    name: "B".to_string(),
                    sla: sla(99.9),
                },
            ],
        };
        let lj = serde_json::to_string(&l).unwrap();

        println!("{:?}", lj);

        let json = r#"{"type":"Serial","items":[{"type":"Single","name":"A","sla":0.999},{"type":"Single","name":"B","sla":0.999}]}"#;

        let deserialized: Item = serde_json::from_str(json).unwrap();

        println!("{:?}", deserialized);
    }
}
