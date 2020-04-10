#![allow(dead_code)]
use std::mem;

struct Point{
    x:f64,
    y:f64
}

fn origin()-> Point
{
    Point{x:12.0,y:10.0}
}

pub fn stack_and_heap(){
    let p1=origin();
    let p2 = Box::new(origin());

    println!("Hello p1 {} bytes",mem::size_of_val(&p1));
    println!("Hello p2 {} bytes",mem::size_of_val(&p2));
    let  p3=*p2;
    println!("{}",p3.x );
}