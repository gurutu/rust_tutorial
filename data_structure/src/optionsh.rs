#![allow(dead_code)]
#![allow(unused_variables)]

pub fn option(){
    //option <T>
     let x =3.0;
     let y =2.0;
     //some(z) None
     let result:Option<f64> =
     if y!= 0.0 { Some(x/y)} else { None };
    println!("{:?}",result );

    match result
    {
        Some(z) => println!("{}/{}={}",x,y,z ),
        None => println!("None" ),

    }
// if let  /while let 
if let Some(z) =result 
{
    println!("z={}",z );
} 

}