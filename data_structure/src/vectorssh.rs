#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


pub fn vector_example(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    let idx:usize =0;
 println!("a[0] = {}",a[idx] );
    println!("a value is {:?}",a );
    while let Some(x) =a.pop()
    {
        println!("print ln {}", x);
    }
}