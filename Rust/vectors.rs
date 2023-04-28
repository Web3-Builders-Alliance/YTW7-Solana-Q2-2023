use std::mem;
pub fn run(){
     let mut numbers:Vec<i32>= vec![1,2,3,4,5];

     //Re-assign value
     numbers[2]=20;

     //adding to vectors
     numbers.push(6);
     numbers.push(7);

     //pop off last value
     numbers.pop();

     println!("{:?}",numbers);


     //get single value
     println!("Single Value: {}",numbers[0]);

     //get array len
     println!("Vector lenght: {}",numbers.len());

     //vectors  are stack allocated
     println!("Vector occupies {} bytes", mem::size_of_val(&numbers));//returns 20 since each i32 element takes 4 bytes
    

     //get slice
     let slice: &[i32] = &numbers[0..2];

     println!("Slice : {:?}",slice);//returns [1,2]

     //loop through vector values
     for x in numbers.iter(){
        println!("Number: {}", x);
     }

     //loop and mutate values
     for x in numbers.iter_mut(){
        *x *=2;
     }

     println!("Number Vec: {:?}",numbers);//returns multiples of 2
}