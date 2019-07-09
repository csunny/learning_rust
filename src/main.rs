use std::io;
use std::cmp::Ordering;
use rand::Rng;
mod basic;
mod mstruct;
// mod ownership;


fn main() {
    // guess a number
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");


        let guess: u32 = guess.trim().parse().expect("please type a number");
        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo You Win");
                break;
            }
        }
    }

    // basic concept  
    basic::basic_concept();    
    basic::compose_type();

    let x = basic::five();
    println!("The value of x is: {}", x);

    basic::for_test();


    // ownership

    // struct
    let user1 = mstruct::User{
        email: String::from("cfqcsunny@gmail.com"),
        username: String::from("csunny"),
        sign_in_count: 1,
        active: true,
    };

    println!("{}", user1.email);

    let rect1 = mstruct::Rectangle {width: 30, height: 50};
    let rect2 = mstruct::Rectangle {width: 10, height: 40};

    println!("The area of the rectangle is {} square pixels.",
            rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));


}
