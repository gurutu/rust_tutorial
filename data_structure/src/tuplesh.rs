#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn tuple_ex(){
    let x= 3;
    let y = 4;
    let sp =sum_and_product(x,y);
    println!("sum {:?}",sp );
    println!("sp0 ={} sp1 ={} ",sp.0,sp.1 );
  //destructuring
  let (a,b)=sp;
  println!("a ={},b ={}",a,b );
  let sp1 =sum_and_product(4,5);
  let combine =(sp,sp1);
  println!("combine= {:?}",combine );
  println!("combine one value = {}",(combine.1).1 );
 let ((c,d),(e,f))=combine;
 println!("c={},d={},e={},f={}",c,d,e,f );

}

fn sum_and_product(x:i32,y:i32)->(i32,i32){
  (x+y,x*y)
}