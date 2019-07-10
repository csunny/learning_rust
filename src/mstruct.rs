use std::collections::HashMap;

#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub struct Rectangle{
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // add code here
    pub fn area(&self) -> u32{
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
} 

pub enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter =>25,
    }
}


pub fn split_text(){
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}