use std::io;
fn main() {
   println!("Student loan repayment estimator");


    println!("what is your principle,p");
    let mut principle = String::new();
    io::stdin().read_line(&mut principle).expect("Failed to take input");
    let P :f32 = principle.trim().parse().expect("didnt get input");


     println!("what is your time,T");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to take input");
    let T:f32 = time.trim().parse().expect("didnt get input");

     println!("what is your rate,R");
    let mut rate = String::new();
    io::stdin().read_line(&mut rate).expect("Failed to take input");
    let R:f32 = rate.trim().parse().expect("didnt get input");

    let A:f32 = P *(1+R/100.0)^T;
    println!("your value for amount :{}", A);
}
