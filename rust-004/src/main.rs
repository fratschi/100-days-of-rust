


fn contact_strings(s1: &mut String, s2: &mut String) -> String {
    s1.push_str(s2);
    return s1.clone();    
}

/* this won't work.
fn concat_macro(s1: &'static str, s2: &'static str) -> &'static str {
    let x= concat!(s1, s2);
    return x;
}
*/

fn format_macro(s1: &str, s2: &str) -> String {
    return format!("{}{}", s1, s2);
}

fn format_macro_string(s1: String, s2: String) -> String {
    return format!("{}{}", s1, s2);
}

fn format_macro_str(s1: &'static str, s2: &'static str) -> String {
    return format!("{}{}", s1, s2);
}




fn main() {

    let mut s1 = String::from("Hello, ");
    let mut s2 = String::from("World!");
    let s3 = contact_strings(&mut s1, &mut s2);
    println!("{:?}",s3);

    let s4 = "test";
    let s5 = format!("{}{}",s1,s4);
    println!("{:?}", s5);
}

