use std::io;


mod sim;
mod config;
use sim::az;

fn main() {
    println!("Hello, world!");

    config::config();


    az::az();
}
