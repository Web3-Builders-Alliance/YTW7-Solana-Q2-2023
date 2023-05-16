pub fn run(){
    let name="Yahya";
    // let age=100;
    let mut age=100;

    println!("my name is {} and my age is {}",name,age);
    
    age=200;
    println!("my name is {} and my age is {}",name,age);


    //Define Constants
    const ID: i32 = 001; //i->signed(-16 to +16 bits) and u->unsigned(0 to 32 bits)
    println!("ID: {}",ID);

    //Assign multiple variables
    let (my_name,my_age)=("Yahya",100);
    println!("{} is {}",my_name,my_age);
}