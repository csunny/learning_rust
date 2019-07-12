pub fn basic_concept(){
    let x = 5;
    println!("The value of x is: {}", x);
}

pub fn compose_type(){
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("five_hundred: {}, six_point_four{}, one {}", five_hundred, six_point_four, one);
}

pub fn five() -> i32{
    5
}

// 控制流
pub fn for_test(){
    let a = [10, 20, 30, 40, 50];

    for e in a.iter(){
        println!("the value is: {}", e);
    }
}