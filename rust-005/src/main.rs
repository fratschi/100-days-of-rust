enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Value {
    Int(i32),
    Float(f32),
    Text(String),
}

enum Node {
    Leaf(String),
    Branch(String,Box<Node>),
}

fn iterate_node(node: &Node) {
    match node {
        Node::Leaf(s) => println!("{}",s),
        Node::Branch(s, n) => {
            println!("Branch: {}", s);
            iterate_node(n);
        }
    }
}

fn main() {
    let direction = Direction::Up;

    match direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }

    let value = Value::Int(5);

    match value {
        Value::Int(i) => println!("Int: {}", i),
        Value::Float(f) => println!("Float: {}", f),
        Value::Text(s) => println!("Text: {}", s),
    }

    let n = Node::Branch(
        "Branch 1".to_string(),Box::new(Node::Branch("Branch 2".to_string(), Box::new(Node::Leaf("Leaf 1".to_string())))),
    );

    iterate_node(&n);


    println!("My name is {first} {last}", first = "John", last = "Smith");

}
