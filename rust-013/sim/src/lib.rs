pub fn print_me() {
    println!("Hello, world!");
}
fn hidden() {
    println!("I'm hidden!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        print_me();
    }
}
