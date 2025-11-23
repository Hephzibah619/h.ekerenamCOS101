fn main() {
    let A:i32 =10;
    let B:i32 = 20;

    println!("Value of a A: {}",A );
    println!("Value of a A: {}",B );

    let mut res = A>B ;
    println!("A greater than B: {}",res);

    res = A<B;
    println!("A lesser than B:{}",res);


    res = A>=B;
    println!("A greater or equal to B:{}",res);

    res = A<=B;
    println!("A lesser than or equal to B:{}",res);

    res = A==B ;
    println!("A is equal to B: {}",res);

    res = A!=B;
    println!("A is not equal to B: {}",res);

}
