
#[derive(Debug,Clone)]
struct Person {
    name: String,
    age: u8,
}

fn create_person(name: &str, age: u8) -> Person {
    
    return Person { 
        name: String::from(name), 
        age: age,
    }
}


fn check_adult(p: &Person) -> bool {
    if p.age > 18 {
        return true;
    } else {
        return false;
    }
}

fn check_adult_clone(p: Person) -> bool {
    if p.age > 18 {
        return true;
    } else {
        return false;
    }
}

fn tuples() -> (String, String) {
    return (String::from("Hello"), String::from("World"));
}

fn tuples_types() -> (String,  &'static str) {
    return (String::from("Hello"),"World");
}


fn main() {
   
   let p = create_person("John", 30);


    println!("{:?}", p);
    // Whats the difference between the two?
    println!("{:?}", p);


    println!("Is person older than 18? {}",check_adult(&p));
    
   
    println!("Is {} older than 18? {}", p.name, check_adult(&p));
   

    println!("Is {} older than 18? {}", p.name, check_adult_clone(p.clone()));
    
    println!("Tuples: {:?}, {:?}", tuples().0 , &tuples().1);

    let (s1, s2) = tuples();

    println!("Tuples: {:?}, {:?}", s1, s2);


    let (t1, t2) = tuples_types();

    println!("Tuples: {:?}, {:?}", t1, t2);

}

