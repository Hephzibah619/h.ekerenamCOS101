
use std::io;
fn main() {

    println!("Rust program ");


   println!("code/titem/t/tprice(N)");
      println!("T/ttea/t/t800");
      println!("C/tcoffee/t/t1200");
         println!("S/tsandwish/t/t2000");
            println!("J/tjuice/t/t1,500");


            // what is the item code

 println!("what is the item code");
   let mut code = String::new();
   io::stdin().read_line(&mut code).expect("Failed to take in input");
   let code = code.trim();
    
    //what is the quantity

    println!("what is the quantity");
   let mut quantity = String::new();
   io::stdin().read_line(&mut quantity).expect("Failed to take in input");
   let quantity:i32 = quantity.trim().parse().expect("failed to take in input");
    
    let price = match code {
        "T"=> 800,
        "C" => 1200,
        "S" => 2000,
        "J" => 1500,
        &_=> todo!(),  
    };
    //total cost
    let total_cost:f32 = (price * quantity) as f32;
     println!("The total cost is before the discount: {}", total_cost);
    
    if total_cost >= 5000.0 {
        let _discount:f32 = (total_cost*0.05) as f32;


    }
    println!("your total is with a _discount:");


}
