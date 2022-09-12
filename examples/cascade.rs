// There’s also the possibility of cascading. e.g.: $field.0.num, it’s experimentally implemented for 
// Strung::strung_dollar and Strung::strung_hashtag at the moment, cause it was the easiest to do. 🦀

use strung::prelude::*;
 
// #[strung(ignore)] just because none of them are implementing Display!
#[derive(Strung)] struct A {#[strung(cascade,ignore)]field:B}
#[derive(Strung)] struct B (u32,#[strung(cascade,ignore)]C);
#[derive(Strung)] struct C {#[strung(cascade,ignore)]field:D,num:u32}
#[derive(Strung)] struct D (#[strung(cascade,ignore)]E);
#[derive(Strung)] struct E {num:u32}
 
fn main(){
    let test = A{
        field: B(500,C{
            num: 123,
            field: D(E{
                num: 623
            })
        })
    };
    let text = test.strung_dollar(
        "Hello, $field.0 + $field.1.num = $field.1.field.0.num"
    );
    println!("{}",&text);
 
    let text = test.strung_hashtag(
        "Hello, #field.0 + #field.1.num = #field.1.field.0.num"
    );
    println!("{}",&text);
}