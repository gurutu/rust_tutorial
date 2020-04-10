#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn stringex(){
    //&str =string slice
    let s:&'static str ="hello world rust";
    let s1:&'static str ="my name praa";
    let s2:&'static str = "my name ";
for c in s.chars().rev()
{
    println!("{}",c );
}
let mut s = String::from("foo");
let mut letters=String::new();
letters.push_str("hello this is object ");//;
println!("lttter {:?}",letters );

let mut u:&str ="hello";
u =" Myhello hello my";
println!("u {}",u );

s=s+"Hello";
    println!("{}",s );
}