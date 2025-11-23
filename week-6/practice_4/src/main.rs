fn main() {
   let fullname = "Chibidum John umeh";
   let department = "Computer Science";
   let uni = "Pan- Atlantic University";

   let mut school = "School of Science".to_string();
   //push string
   school.push_str("and Technology");

   println!("My name is: {}",fullname );
   //check length 
  println!("the length my fullnam is:{}", fullname.len());
  println!("I am a student of {} Department ", department);
  println!("{}",school);
  println!("{}",uni);

}
