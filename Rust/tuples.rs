pub fn run()
{
    //tuples cn carry max 12 elements

    let person: (&str, &str, i8)= ("Yahya","WBA",100);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}