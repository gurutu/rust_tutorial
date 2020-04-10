#![allow(dead_code)]
#![allow(unused_variables)]

pub fn enumiration(){
let c:Color =Color::RgbColor(0,0,0);
match c
{
   Color::Red => println!("Red" ),
   Color::Green => println!("Green"),
   Color::Blue => println!("Blue "),
   Color::RgbColor(0,0,0)=>println!("Black"),
   Color::RgbColor(r,g,b)=>println!("Black"),
   Color::Cmyk{cyan:_,magenta:_,yello:_,black:_}=>println!("Black"),
   
}
}

enum Color{
  Red,
  Green,
  Blue,
  RgbColor(u8,u8,u8),//tuple
  Cmyk{cyan:u8,magenta:u8,yello:u8,black:u8}//struct
} 