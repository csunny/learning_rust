
// 所有权 是Rust最为与众不同的特性，它让Rust无需垃圾回收即可保障内存安全。 因此，理解Rust中所有权
// 如何工作是十分重要的。  
fn main() {
    println!("Hello, world!");

    let s = String::from("hello");
    println!("string {}", s);        
}
