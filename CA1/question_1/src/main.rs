use std::io;


fn main() {
      println!("units_consumed(KWh)/t/Rate per unit(N)");
      println!("0-100/t/20)");
      println!("101-300/t/35");
      println!("301 and above/t/50");

      //what is your name
      println!("what is your name");
      let mut name = String::new();
      io::stdin().read_line(&mut name).expect("Failed to take input");
      let name:i32 =name.trim().parse().expect("failed to get your name");
   
      //what are the units consumed
       println!("what are the units consumed");
      let mut units = String::new();
      io::stdin().read_line(&mut units).expect("Failed to take input");
      let units:f32 =units.trim().parse().expect("failed the units consumed");
       
       let units = match units_consumed{
        "0-100" => 20,
        "101-300" => 35,
        "301>" => 50,
        &_=> todo!(), 


       };

       if units:f32 >500.00 && 50000{
        println!("input the rate consumed:{}",units);

       };
       else {
        println!("input the rates consumed:{}"units);
       };

       let total_bill = (units) * (units_consumed);
       println!("your total bill is: {}", total_bill);
}
