#![allow(dead_code)]

pub fn looping(){
    let  mut x=1;
    while x < 100
    {
        x*=2;
        if x==16 {continue;}
        println!("x = {}",x );
    }
//loop
let mut y =1;

loop
{
    y *=2;
    println!("y = {}",y );
    break;

}

//for loop

for x in 1..11 {
    println!("x = {}",x );
}

for (pos,y) in (30..41).enumerate()
{
    println!("y = {} and position is {}", y,pos);
}

  
}