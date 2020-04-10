#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn slice_ex(){

let mut data =[1,2,3,4,5];
use_slice(&mut data[1..4]);
println!("{:?}",data );

}

fn use_slice(slice:&mut [i32]){

println!("first element {} , len ={} ",
slice[0],slice.len() );
slice[0]=123;
}
