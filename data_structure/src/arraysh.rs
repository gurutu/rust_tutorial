#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

//if you know size use like that
pub fn array_example(){
    //5 is size i32 is a type 
    let mut a:[i32;5]=[1,2,3,4,5];
    println!(" a has {} elemnet ,first valiis 
    {}",a.len(),a[0] );
    a[0]=123;
    println!("a[0] = {}",a[0] );

    println!("{:?}",a );

    if a ==[123,2,3,4,5]
    {
        println!("do not match ");
    }
    for i in a.iter()
    {
        println!("a = {}",i );
    }

}