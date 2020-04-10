#![allow(dead_code)]

struct Point{
 x:f64,
 y:f64
}
struct Line{
    start :Point,
    end : Point
}

fn structure()
{
let p =Point{x:3.0,y:4.0};
println!("p.x  = {} ,p.y ={}",p.x,p.y );
let p1= Point{x:9.0,y:10.0};
let _myline=Line{start:p,end:p1};

}

pub fn structfuntion(){
    structure();
}