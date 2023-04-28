pub fn run(){
    println!("Hello from the print.rs file!");
    
    //Basic Formatting
    println!("Number: {}",1); 
    println!("{} is from {}","Yahya","India");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}","Yahya","India","Code");

    //Named Arguments
    println!("{name} likes to play {activity}",name="Yahya",activity="Football");

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    //Placeholder for debug Traits(used for printing multiple entities)
    println!("{:?}", (12,true,"Hello"));


    //Basic Maths
    println!("10 + 10 ={}",10+10);


}