use std::mem;
pub fn run(){
     let mut numbers:[i32; 5]= [1,2,3,4,5];

     //Re-assign value
     numbers[2]=20;

     println!("{:?}",numbers);


     //get single value
     println!("Single Value: {}",numbers[0]);

     //get array len
     println!("Array lenght: {}",numbers.len());

     //arrays are stack allocated
     println!("Array occupies {} bytes", mem::size_of_val(&numbers));//returns 20 since each i32 element takes 4 bytes
    

     //get slice
     let slice: &[i32] = &numbers[0..2];

     println!("Slice : {:?}",slice);//returns [1,2]

}