struct Point{
    x:f32,
    y:f32
}

struct Line{
    start:Point,
    end:Point
}

impl Line
{
    fn len(&self)-> f32
    {
        let dx=self.start.x-self.end.x;
        let dy =self.start.y-self.end.y;
        (dx*dx+dy*dy).sqrt()

    }
}
fn say_hello(){ println!("Sya hello");}

//clouser
fn clouser()
{
    let sh=say_hello;
    sh();
    let plus_one=|x:i32| -> i32 {x+1};
    let a=9;
    println!("{}+1={}",a,plus_one(a) );
    let plus_two =|x|
    {
        let mut z= x;
        z +=2;
        z
    };

    println!("{}+1={}",3,plus_two(3) );
}
 
 pub fn function_def(){
     println!(" sum x and y = {}",sum(2,3) );
     println!(" sum x and y = {}",withoutreturn(2.9,3.4) );
     let p=Point{ x:3.0,y:4.0};
     let p1=Point{ x:5.0,y:10.0};
     let myline=Line{start:p,end:p1 };
     println!("Line length {}",myline.len() );
     clouser();

 }

 fn sum(x:i32,y:i32) -> i32{
     return  x+y ;
 }

 fn withoutreturn(x:f32,y:f32) -> f32{
      x+y 
}

