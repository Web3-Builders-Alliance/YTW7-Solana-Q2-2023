pub fn run(){
    // let hello=String::from("Hello");
    let mut hello=String::from("Hello ");

    //get length
    println!("Length: {}",hello.len());//returns 5

    //pushing a char
    hello.push('W');

    //pushing a string
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());//returns 12

    //Check if empty
    println!("Is empty : {}",hello.is_empty());

    //check containg words
    println!("Contains 'World' {}", hello.contains("World"));

    //replace
    println!("Replace: {}",hello.replace("World","WBA"));

    //loop through strings
    for word in hello.split_whitespace(){
        println!("{}",word);
    }

    //Create string with capacity
    let mut s= String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2,s.len());
    assert_eq!(10, s.capacity());

    println!("{}",s);



    println!("{}",hello);
}