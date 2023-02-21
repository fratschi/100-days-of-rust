use log::error;



fn parse_date_a(date: &str) -> chrono::DateTime<chrono::Utc> {
    date.parse::<chrono::DateTime<chrono::Utc>>().unwrap()
}


fn parse_date_b(date: &str) -> chrono::DateTime<chrono::Utc> {
   match  date.parse::<chrono::DateTime<chrono::Utc>>() {
       Ok(d) => d,
       Err(e) => {
        error!("{}",e);
        chrono::Utc::now()
    },
   }
}



fn main() {
    //  init logging
    env_logger::init();

    let d1 = "2023-01-01T00:00:00Z";

    println!("parsed_date: {}", parse_date_a(d1));

    let d2 = "2023-01-01T00:00:00X";

    println!("parsed_date: {}", parse_date_b(d2).format("%Y-%m-%d"));
    println!("parsed_date: {}", parse_date_b(d2).to_rfc3339());

}
