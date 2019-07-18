use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // for val in v1_iter{
    //     println!("Got: {}", val);
    // }
    
    assert_eq!(v1_iter.next(), Some(&1));

   let handle = thread::spawn(||{
       for i in 1..10{
           println!("hi number {} from the spawned thread!",  i);
           thread::sleep(Duration::from_micros(1));
       }
   });
   
    for i in 1..5{
       println!("hi number {} from the main thread", i);
       thread::sleep(Duration::from_millis(1));
    } 

    handle.join().unwrap();


    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
