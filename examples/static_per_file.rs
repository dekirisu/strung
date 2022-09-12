// Import everything from prelude
use strung::prelude::*; 

// Shadow static pre/postfix for this file.
strung::config::static_on_file!("[","]");       


#[derive(Strung)]                   
struct Test {
    text: &'static str,
    num: u32
}

fn main(){
    let test = Test {
        text: "5k",
        num: 5000
    };
    // Use whatever you've set above
    let text = test.strung_static("[text]=[num]");  
    println!("{}",&text);
}