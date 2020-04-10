#![allow(dead_code)]

pub fn if_statement(){
    let temp =26;
    if temp <30{
        println!("print temp {} ",temp );
    }else if temp >30 {
        println!("println temp {}",temp );

    }else{
        println!("println temp {}",temp );
    } 


    let day =if temp>20 {"sunny"} else {"cloudy"};
    println!("day {} =",day );
    println!("is it  {} ",if temp >20 {"hot"} 
    else if temp<20 { "cold"} else {"Ok"} );
}