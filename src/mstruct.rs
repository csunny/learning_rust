#[derive(Debug)]
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


