fn main() {
    println!("Hello, world!");

    let x = 2.0;

    let y :f32 = 3.0;
    let sum = x + y;
    println!("两个数的和为:sum {}",sum);

    let months = ["January", "February", "March", "April", "May", "June",
     "July", "August", "September", "October","Noveber", "December"];

    let i = 0;
    for e in months.iter(){
        println!("The month is:{}", e);
    } 

    for number in (1..4).rev(){
        println!("{}!", number);
    }

    for n in (1..4){
        println!("{}", n);
    }
}
