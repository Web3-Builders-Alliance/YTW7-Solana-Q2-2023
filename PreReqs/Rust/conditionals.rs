pub fn run(){
    let age=19;
    let check_id:bool=false;
    let knows_person_of_age:bool=true;

    if age>=21 && check_id || knows_person_of_age{
        println!("Eligible for a drink!");
    }
    else if age<21 && check_id
    {
        println!("Not eligible for a drink!");
    }
    else
    {
        println!("Show your ID");
    }


    //shorthand if
    let is_of_age= if age>=21 {true} else {false};
     println!("Is of age: {}",is_of_age);

}