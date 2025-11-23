fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 &n3 refernece is passed

    // About Electical/electronic
    println!("\n ThE {}IS INFORMED bY THE ASPIRATION TO TRAIN ELECTICAL/ELECTRONIC ENGINEERING PROFESSIONALS IN THE AREAS OF DESIGN. BUILDING AND MAINTENANCE OF ELCTRICAL CONTROL SYSTEMS,",n4 );

    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2; // w2 reference is passed 
    println!();
    println!("{} is aimed at developing compinent, reative, innovative, entrepreneurial and ethically-minded persons, capable of creating value in the driverse fields of ComputerScience.", w3);

}
