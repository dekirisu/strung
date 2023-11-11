#![allow(non_snake_case,dead_code)]
use strung::prelude::*; // import everything from prelude
 
// named struct
#[derive(Strung)]   // easy derive
struct Test {
    num: u32,
    name: &'static str,
}
 
// tuple struct
#[derive(Strung)]   // easy derive
struct TestTup(u32,&'static str);                 
 
// custom pre/postfix per struct + ignore
#[derive(Strung)]   // easy derive
#[strung("%","%")]  // custom pre/postfix!
struct TestCustom {
    num: u32,
    // ignore: makes this field unavailable
    // this would fail w/o the ignore, cause no [Display]!
    // other usage: gain a lil more performance
    #[strung(ignore)] nop: NoDsply  
}
 
// custom pre/postfix per struct + ignore
#[derive(Strung)]   // easy derive
struct TestCascade {
    // cascade: makes the fields of another Strung available
    // the ignore only affects the the struct itself, not its fields
    // and this would fail w/o it cause it doesn't implement it!
    #[strung(cascade,ignore)]
     tup: TestTup  
}
 
// struct with no Display trait
struct NoDsply;

fn main(){
 
    // create structs!
    let NAMED  = Test {num: 1, name: "st"};
    let TUPLE  = TestTup (1, "st");
    let CUSTOM = TestCustom {num: 1, nop: NoDsply};
 
    // most general - you'll probably mostly use this! - using {field_name}
    let text = NAMED.strung("{num}{name}"); 
    println!("strung: {}",&text);
     
    // also works with String, just reference it
    let s: String = "{num}{name}".into();
    let text = NAMED.strung(&s); 
    println!("strung with String: {}",&text);
 
    // it will always replace every occurrence
    let text = NAMED.strung("{num}{num}th < {num}{name}"); 
    println!("strung multi: {}",&text);
 
    // for tuple structs, use the fields index number, instead of the name
    let text = TUPLE.strung("{0}{1}"); 
    println!("strung tuple: {}",&text);
     
    // the [strung] function will change if you set custom pre/postfix - see TestCustom above
    let text = CUSTOM.strung("%num%st"); 
    println!("strung custom per struct: {}",&text);
 
    // there are different presets, so you can still use {field_name} using [strung_curly]
    let text = CUSTOM.strung_curly("{num}st"); 
    println!("strung_curly: {}",&text);
 
    // note: {nop} is not available, cause it's ignored - see TestCustom above
    let text = CUSTOM.strung_curly("{num}st {nop}"); 
    println!("strung ignored: {}",&text);
 
    // [strung_dollar] for $field_name
    let text = NAMED.strung_dollar("$num$name");
    println!("strung_dollar: {}",&text);
 
    // [strung_dollry] for ${field_name}
    let text = NAMED.strung_dollry("${num}${name}");
    println!("strung_dollry: {}",&text);
 
    // [strung_hashtag] for #field_name
    let text = NAMED.strung_hashtag("#num#name");
    println!("strung_hashtag: {}",&text);
 
    // most flexible - inline setting via [strung_dynamic] - a bit less efficient
    let text = NAMED.strung_dynamic("<",">","<num><name>");
    println!("strung_dynamic: {}",&text);
 
    // also flexible - global static variables, you can easily change ...
    strung::config::static_global("+","+");
    let text = NAMED.strung_static("+num++name+");
    println!("strung_static[1]: {}",&text);
 
    // ... whenever you want, but usually you'll just need it once at the start of the main
    strung::config::static_global("[","]");
    let text = NAMED.strung_static("[num][name]");
    println!("strung_static[2]: {}",&text);
 
    // [strung_hashtag] and [strung_dollar] also enable cascading
    let CASCADE = TestCascade {tup: TestTup(2,"nd")};
    let text = CASCADE.strung_dollar("$tup.0$tup.1");
    println!("strung_dollar cascading: {}",&text);
 
    // [strung_hashtag] and [strung_dollar] also enable cascading
    let CASCADE = TestCascade {tup: TestTup(2,"nd")};
    let text = CASCADE.strung_hashtag("#tup.0#tup.1");
    println!("strung_hashtag cascading: {}",&text);

    // [strung_hashtag] and [strung_dollar] also enable cascading
    let CASCADE = TestCascade {tup: TestTup(2,"nd")};
    let text = CASCADE.strung_curly("{tup.0}{tup.1}");
    println!("strung_curly cascading: {}",&text);

    // [strung_hashtag] and [strung_dollar] also enable cascading
    let CASCADE = TestCascade {tup: TestTup(2,"nd")};
    let text = CASCADE.strung_angle("<tup.0><tup.1>");
    println!("strung_angle cascading: {}",&text);
    
    // [strung_hashtag] and [strung_dollar] also enable cascading
    let CASCADE = TestCascade {tup: TestTup(2,"nd")};
    let text = CASCADE.strung_dollry("${tup.0}${tup.1}");
    println!("strung_dollry cascading: {}",&text);
    
    // [strung_hashtag] and [strung_dollar] also enable cascading
    let CASCADE = TestCascade {tup: TestTup(2,"nd")};
    let text = CASCADE.strung("{tup.0}{tup.1}");
    println!("strung cascading: {}",&text);
 
    // most flexible - inline setting via [strung_dynamic] - a bit less efficient
    let text = CASCADE.strung_dynamic("<",">","<tup.0><tup.1>");
    println!("strung_dynamic cascading: {}",&text);

    // also flexible - global static variables, you can easily change ...
    strung::config::static_global("+","+");
    let text = CASCADE.strung_static("+tup.0++tup.1+");
    println!("strung_static cascading: {}",&text);

}