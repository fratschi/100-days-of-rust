
#[derive(Debug,Clone)]
struct Person {
    name: String,
    age: u8,
    address: Option<Address>,
}

#[derive(Debug,Clone)]
struct Address {
    street: String,
    number: u8,
}


fn main() {
    
    let p = Person {
        name: "fratschi".to_string(),
        age: 50,
        address: None,
    };


    println!("{:?}", p);

    let mut p2 = p.clone();

    p2.name = "fratschi2".to_string();
    p2.age = 51;
    p2.address = Some(Address {
        street: "street".to_string(),
        number: 1,
    });

    println!("{:?} {:?}", p, p2);

}
