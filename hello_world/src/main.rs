mod sh;
mod ifsh;
mod looping;
mod matchsh;
use std::mem;

fn main() {
    primitive_data_type();
    operater();
    sh::stack_and_heap();
    ifsh::if_statement();
    looping::looping();
    matchsh::match_statement();
}

fn operater(){
    println!("\nThis is arithmetic opration \n");
//arithmetic
let mut a = 2+3*4;
println!("print a ={}",a );
//rust not support  ++ --
a=a+1;
a-=2;
println!("Checking incren=ment a = {}",a );
//bitwise oprater 
let  c= 1|2;
println!(" 1| 2 ={}",c );
let two_to_10= 1<<10;
println!("2 ^10={}",two_to_10 );



}

fn primitive_data_type(){
     //this value is immutable value 
     let  a:u8 = 123;
     //a=129;
     println!("a = {}",a);
     //this value is mutabe value 
     let mut  b:u8=0;
     println!("b ={}",b);
     b=12;
     // we are n and making and
     println!("b = {}",b);
     println!("Hello, world!");
 let  mut c= 12334;
 c=2436546+c;
 println!("c ={} ,size ={}",c,mem::size_of_val(&c));
 //i8 u8 i16 u16 i32 u32 i64 u64
  let z:isize=123;//isize and usize 
 
 let size_of_z= mem::size_of_val(&z);
  println!("Hello z = {},{},{}",z,size_of_z,size_of_z*8);
 
 let d:char ='x';
 println!("Hello z = {},{}",d,mem::size_of_val(&d));
 let e:bool =false;
 println!("Hello z = {},{}",e,mem::size_of_val(&e));
}
