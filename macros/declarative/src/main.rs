use std::collections::HashMap;
use declarative::{hello, map};

fn main() {
    hello!();
    let months =  map!(i32, String);


    let months2 =  map!(
        1 => "Enero".to_owned(),
        2 => "Febrero".to_owned(),
        3 => "Marzo".to_owned()
    );

    println!("{:?}", months);
    println!("{:?}", months2);
}
