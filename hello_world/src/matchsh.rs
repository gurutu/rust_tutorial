pub fn match_statement(){
    let  country_code=100;
    let conntry =match country_code
    {
        44|45 => "uk",
        46...50 => "Sweden",
        91..=100  => "india",
        _  => "unknow"
    };
    println!("the country with  code {} is {}",country_code,conntry );
}