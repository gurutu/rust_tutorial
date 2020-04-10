#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]


pub fn union_fn(){
    let mut iof= IntOrFloat{i:123};
    unsafe {iof.i=42;}
      let value =unsafe {iof.i};
      processing(iof);
      processing(IntOrFloat{f:5.9});
}

fn  processing(iof:IntOrFloat){
unsafe{
    match iof
    {
        IntOrFloat { i:42 }  =>
         { println!("meaning of life" )},
        IntOrFloat { f } => { println!("f32 = {}",f )},

    }
}
}


union IntOrFloat
{
    i:i32,
    f:f32
}