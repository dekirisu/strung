//! String formatter/builder with easy access of struct fields, which implement the [std::fmt::Display] trait.
//! If they do not, they can be marked to be ignored.
//! # Usage
//! Here's most you have to know!
//! ```
//! use strung::prelude::*; // import everything from prelude
//! 
//! fn main(){
//! 
//!     // create structs - defined below cause instant action!
//!     let NAMED  = Test {num: 1, name: "st"};
//!     let TUPLE  = TestTup (1, "st");
//!     let CUSTOM = TestCustom {num: 1, nop: NoDsply};
//! 
//!     // most general - you'll probably mostly use this! - using {field_name}
//!     let text = NAMED.strung("{num}{name}"); 
//!     assert_eq!(&text,"1st");
//!     
//!     // also works with String, just reference it
//!     let s: String = "{num}{name}".into();
//!     let text = NAMED.strung(&s); 
//!     assert_eq!(&text,"1st");
//! 
//!     // it will always replace every occurrence
//!     let text = NAMED.strung("{num}{num}th < {num}{name}"); 
//!     assert_eq!(&text,"11th < 1st");
//! 
//!     // for tuple structs, use the fields index number, instead of the name
//!     let text = TUPLE.strung("{0}{1}"); 
//!     assert_eq!(&text,"1st");
//!     
//!     // the [strung] function will change if you set custom pre/postfix - see TestCustom below
//!     let text = CUSTOM.strung("%num%st"); 
//!     assert_eq!(&text,"1st");
//! 
//!     // there are different presets, so you can still use {field_name} using [strung_curly]
//!     let text = CUSTOM.strung_curly("{num}st"); 
//!     assert_eq!(&text,"1st");
//! 
//!     // note: {nop} is not available, cause it's ignored - see TestCustom below
//!     let text = CUSTOM.strung_curly("{num}st {nop}"); 
//!     assert_eq!(&text,"1st {nop}");
//! 
//!     // [strung_dollar] for $field_name
//!     let text = NAMED.strung_dollar("$num$name");
//!     assert_eq!(&text,"1st");
//! 
//!     // [strung_dollry] for ${field_name}
//!     let text = NAMED.strung_dollry("${num}${name}");
//!     assert_eq!(&text,"1st");
//! 
//!     // [strung_hashtag] for #field_name
//!     let text = NAMED.strung_hashtag("#num#name");
//!     assert_eq!(&text,"1st");
//! 
//!     // [strung_angle] for <field_name>
//!     let text = NAMED.strung_angle("<num><name>");
//!     assert_eq!(&text,"1st");
//! 
//!     // most flexible - inline setting via [strung_dynamic] - a bit less efficient
//!     let text = NAMED.strung_dynamic("<",">","<num><name>");
//!     assert_eq!(&text,"1st");
//! 
//!     // also flexible - global static variables, you can easily change ...
//!     strung::config::static_global("+","+");
//!     let text = NAMED.strung_static("+num++name+");
//!     assert_eq!(&text,"1st");
//! 
//!     // ... whenever you want, but usually you'll just need it once at the start of main()
//!     strung::config::static_global("[","]");
//!     let text = NAMED.strung_static("[num][name]");
//!     assert_eq!(&text,"1st");
//! 
//!     // [strung_hashtag] and [strung_dollar] also enable cascading
//!     let CASCADE = TestCascade {tup: TestTup(2,"nd")};
//!     let text = CASCADE.strung_dollar("$tup.0$tup.1");
//!     assert_eq!(&text,"2nd");
//! 
//! }
//! 
//! // named struct
//! #[derive(Strung)]   // easy derive
//! struct Test {
//!     num: u32,
//!     name: &'static str,
//! }
//! 
//! // tuple struct
//! #[derive(Strung)]   // easy derive
//! struct TestTup(u32,&'static str);                 
//! 
//! // custom pre/postfix per struct + ignore
//! #[derive(Strung)]   // easy derive
//! #[strung("%","%")]  // custom pre/postfix!
//! struct TestCustom {
//!     num: u32,
//!     // ignore: makes this field unavailable
//!     // this would fail w/o the ignore, cause no [Display]!
//!     // other usage: gain a lil more performance
//!     #[igno] // or #[strung(igno)] 2b more specific
//!     nop: NoDsply 
//! }
//! 
//! // custom pre/postfix per struct + ignore
//! #[derive(Strung)]   // easy derive
//! struct TestCascade {
//!     // cascade: makes the fields of another Strung available
//!     // automatically ignores the struct itself, not its fields
//!     #[cscd] //, #[cascade], #[strung(cascade)] or #[strung(cscd)]
//!      tup: TestTup  
//! }
//! 
//! // struct with no Display trait
//! struct NoDsply;    
//! ```
//! 
//! # About Statics
//! Prelude imports two static variables [config::STRUNG_PRE] and [config::STRUNG_POST], which can be used
//! to set the prefix and postfix as a configuration. [Strung::strung_static] uses anything called STRUNG_PRE or STRUNG_POST
//! on the file. 
//! 
//! [config::static_global] changes these variables, as you saw in the walkthrough, there's another method of
//! changing it per file. It's not included in the walkthrough cause it shadows these variables, it's a macro called [config::static_on_file].
//! 
//! 📝 Note: This will <strong>maybe</strong> change in future, so these variables dont have to always be imported.
//! 
//! But: here's how it can be used for now:
//! ```
//! // Import everything from prelude
//! use strung::prelude::*;       
//! // Shadow static pre/postfix for this file.              
//! strung::config::static_on_file!("[","]");   
//! 
//! #[derive(Strung)]                        
//! struct Test {
//!     text: &'static str,
//!     num: u32
//! }
//! fn main(){
//!     // Create struct as usual
//!     let test = Test {                       
//!         text: "5k",
//!         num: 5000
//!     };
//!     // Use whatever you've set above
//!     let text = test.strung_static("[text]=[num]");  
//!     assert_eq!(&text,"5k=5000");
//! }
//! ```
//! # Ignore fields
//! Sometimes you wanna ignore certain fields - e.g. in these scenarios:
//! - Get even moar efficiency 📈
//! - A field-type doesn't implement [std::fmt::Display]
//! This can be done with the `#[igno]` attribute, or if it interferes with other
//! macros, you can be more specific: `#[strung(ignore)]` or `#[strung(igno)]`:
//! ```
//! // Import everything from prelude
//! use strung::prelude::*;
//! // A struct, not impl Display
//! struct CustomField (u32);                   
//! 
//! #[derive(Strung)]
//! struct Test {
//!     num: u32,
//!     // Would fail without the attribute:
//!     #[igno] nope: CustomField     
//! }
//! 
//! #[derive(Strung)]
//! struct TestTup (
//!     u32, 
//!     // Would fail without the attribute:
//!     #[strung(ignore)] CustomField,          
//!     &'static str
//! ); 
//! 
//! fn main(){
//!     /* ------------------------------ Named Fields ------------------------------ */
//!     // Create struct as usual
//!     let test = Test {                               
//!         num: 1,
//!         nope: CustomField(0), 
//!     };
//!     // {nope} not available!
//!     let text = test.strung("Number {num} {nope}");  
//!     assert_eq!(&text,"Number 1 {nope}");
//! 
//!     /* ------------------------- Unnamed Fields (Tuple) ------------------------- */
//!     // Create struct as usual
//!     let test = TestTup(1,CustomField(0),":)");    
//!     // {1} not available!  
//!     let text = test.strung("Number {0} {1} {2}");   
//!     assert_eq!(&text,"Number 1 {1} :)");
//! }
//! ```
//! 
//! # ❗ Experimental: Cascading ❗
//! There's also the possibility of cascading. e.g.: `$field.0.num`, it's experimentally implemented for [Strung::strung_dollar] and [Strung::strung_hashtag] at the moment,
//! cause it was the easiest to do. 🦀
//! 
//! For this to work, the field-type has to derive [Strung] via derive macro and mark it with the `#[cascade]`, `#[cscd]` attribute or `#[strung(cascade)]`, `#[strung(cscd)]` in more specific use cases:
//! ```
//! use strung::prelude::*;
//! 
//! // #[strung(ignore)] just because none of them are implementing Display!
//! #[derive(Strung)] struct A {#[cscd]field:B}
//! #[derive(Strung)] struct B (u32,#[cascade]C);
//! #[derive(Strung)] struct C {#[strung(cascade)]field:D,num:u32}
//! #[derive(Strung)] struct D (#[strung(cscd)]E);
//! #[derive(Strung)] struct E {num:u32}
//! 
//! fn main(){
//!     let test = A{
//!         field: B(500,C{
//!             num: 123,
//!             field: D(E{
//!                 num: 623
//!             })
//!         })
//!     };
//!     let text = test.strung_dollar(
//!         "Hello, $field.0 + $field.1.num = $field.1.field.0.num"
//!     );
//!     assert_eq!(&text,"Hello, 500 + 123 = 623");
//! 
//!     let text = test.strung_hashtag(
//!         "Hello, #field.0 + #field.1.num = #field.1.field.0.num"
//!     );
//!     assert_eq!(&text,"Hello, 500 + 123 = 623");
//! }
//! ```
//! 

/// Designed to be used with the strung-derive crate!
pub trait Strung {
    /// Default text replacement via `{field_name}`, changable via `#[strung("pre","post")]`
    /// This is the most efficient cause pre- anf postfixes are merged with the field-names on compilation time!
    fn strung(&self, _text: &str) -> String {todo!()}
    /// Replacement with custom static postfixes and prefixes
    /// - STRUNG_PRE for the prefix - Default: "$"
    /// - STRUNG_POST for the postfix - Default: ""
    /// Therefore by default replacement via `$field_name`.
    /// 
    /// Easy changable:
    /// - File scope (shadowing): [config::static_on_file]
    /// - Global scope (mut): [config::static_global]
    fn strung_static(&self, _text: &str) -> String {todo!()}
    /// Replacement with custom inline Postfixes and Prefixes
    fn strung_dynamic(&self, _pre: &str, _post:&str, _text: &str) -> String {todo!()}
    /// Replacement with custom generic constant char Postfixes and Prefixes
    fn strung_generic <const A:char,const Z:char>(&self, _text: &str) -> String {todo!()}
    /// Same as [Strung::strung] but not changable and always addressable by `{field_name}`
    fn strung_curly(&self, _text: &str) -> String {todo!()}
    /// Same as [Strung::strung] but not changable and always addressable by `$field_name`
    fn strung_dollar(&self, _text: &str) -> String {todo!()}
    /// Same as [Strung::strung] but not changable and always addressable by `${field_name}`
    fn strung_dollry(&self, _text: &str) -> String {todo!()}
    /// Same as [Strung::strung] but not changable and always addressable by `#field_name`
    fn strung_hashtag(&self, _text: &str) -> String {todo!()}
    /// Same as [Strung::strung] but not changable and always addressable by `<field_name>`
    fn strung_angle(&self, _text: &str) -> String {todo!()}
}

/// Just an empty unit struct with Strung trait!
pub struct StrungUnit;
impl Strung for StrungUnit {
    fn strung_dynamic(&self, _pre: &str, _post:&str, text: &str) -> String {text.into()}
    fn strung(&self, text: &str)            -> String {text.into()}
    fn strung_static(&self, text: &str)     -> String {text.into()}
    fn strung_curly(&self, text: &str)      -> String {text.into()}
    fn strung_dollar(&self, text: &str)     -> String {text.into()}
    fn strung_dollry(&self, text: &str)     -> String {text.into()}
    fn strung_hashtag(&self, text: &str)    -> String {text.into()}
    fn strung_angle(&self, text: &str)      -> String {text.into()}
    fn strung_generic <const STRUNG_PRE:char,const STRUNG_POST:char>(&self, text: &str) -> String {text.into()}
}

pub mod config {
    //! Configurations, currently just static pre- and postfix! 👻

    /// Prefix used by [super::Strung::strung_static]
    pub static mut STRUNG_PRE: &'static str = "$";
    /// Postfix used by [super::Strung::strung_static]
    pub static mut STRUNG_POST: &'static str = "";

    /// Quickly shadow [STRUNG_PRE] and [STRUNG_POST] for the current file.
    /// 
    /// Use it after importing the prelude:
    /// ```
    /// use strung::prelude::*;
    /// strung::config::static_on_file!("{","}");
    /// // ...
    /// ```
    #[macro_export]
    macro_rules! static_on_file {($pre: expr, $post: expr) => {
        const STRUNG_PRE: &'static str = $pre;
        const STRUNG_POST: &'static str = $post;
    };}
    pub use static_on_file;

    /// Quickly change [STRUNG_PRE] and [STRUNG_POST] globally.
    /// 
    /// Use anywhere within the runtime:
    /// ```
    /// use strung::prelude::*;
    /// fn main(){
    ///     strung::set_static("{","}");
    ///     // ...
    /// }
    /// // ...
    /// ```
    ///     
    pub fn set_static(pre: &'static str, post: &'static str){
        unsafe {
            STRUNG_PRE = pre;
            STRUNG_POST = post;
        }
    }
    /// alias for [set_static]
    pub fn static_global(pre: &'static str, post: &'static str){
        set_static(pre,post)
    }

}

pub use config::set_static;

pub mod prelude {
    //! All needed goods!
    pub use strung_derive::*;
    pub use super::{Strung,StrungUnit};
    pub use super::config::{STRUNG_PRE,STRUNG_POST};
}