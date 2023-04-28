pub fn run()
{
    let x=1;
    //default is "i32"

    let y=2.4;
    //default is "f64"

    let z:i64=444444444444;
    //explicitly typed

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //BOOLEAN
    let is_active:bool = true;

    //extracting boolean from expression
    let is_greater:bool=10<5;

    let a1="a";
    let emoji='\u{1F600}';

    println("{:?}", (x,y,z,is_active,is_greater,a1,emoji));

}