enum Item {
    Single { name: String, sla: f32 },
    Linear { items: Vec<Item> },
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
        Item::Linear { items: i } => {
            let mut inner = 1f32;
            for item in i {
                inner *= calculate_sla(item);
            }
            sla *= inner;
        }
        Item::Parallel { items: i } => {
            let mut inner = 1f32;
            for item in i {
                inner *= (1f32 - calculate_sla(item));
            }
            sla *= 1f32 - inner;
        }
    }

    return sla;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple() {
        let system = Item::Single {
            name: "System".to_string(),
            sla: sla(99.99),
        };

        assert_eq!(calculate_sla(&system), 0.9999);
    }

    #[test]
    fn test_linear() {
        let linear = Item::Linear {
            items: vec![
                Item::Single {
                    name: "System".to_string(),
                    sla: sla(99.95),
                },
                Item::Single {
                    name: "Database".to_string(),
                    sla: sla(99.95),
                },
            ],
        };
        assert_eq!(calculate_sla(&linear), 0.9990002);
    }

    #[test]
    fn test_parallel() {
        let parallel = Item::Parallel {
            items: vec![
                Item::Single {
                    name: "System".to_string(),
                    sla: sla(99.95),
                },
                Item::Single {
                    name: "Database".to_string(),
                    sla: sla(99.95),
                },
            ],
        };
        assert_eq!(calculate_sla(&parallel), 0.99999976);
    }

    #[test]
    fn test_complex() {
        let complex = Item::Linear {
            items: vec![
                Item::Single {
                    name: "Firewall".to_string(),
                    sla: sla(99.95),
                },
                Item::Single {
                    name: "App Gateway".to_string(),
                    sla: sla(99.95),
                },
                Item::Parallel {
                    items: vec![
                        Item::Single {
                            name: "VM 1".to_string(),
                            sla: sla(95.0),
                        },
                        Item::Single {
                            name: "VM 2".to_string(),
                            sla: sla(95.0),
                        },
                    ],
                },
            ],
        };
        assert_eq!(calculate_sla(&complex), 0.9965027);
    }
}
