<p align="center">
<img src="https://user-images.githubusercontent.com/78398528/282229773-1b280573-6efd-4604-a739-320b4ceb6ef3.gif">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/strung" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/strung-ee6677">
    </a>
    <a href="https://crates.io/crates/strung" style="position:relative">
        <img src="https://img.shields.io/crates/v/strung">
    </a>
    <a href="https://docs.rs/strung" style="position:relative">
        <img src="https://img.shields.io/docsrs/strung">
    </a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative">
        <img src="https://img.shields.io/discord/515100001903312898">
    </a>
</p>

## Easy access to struct fields in strings
🐠 add strung to the dependencies in the `Cargo.toml`:
```toml
[dependencies]
strung = "0.1"
```
🦀 use/import everything of the prelude in rust:
```rust 
use strung::prelude::*;
```
🦊 works for **unnamed** fields:
```rust 
#[derive(Strung)]
struct Struct (&'static str, u32);
fn main(){
    let s: String = Struct("Bob", 10).strung("{0} is {1}th"); 
}
```
🦊 works for **named** fields:
```rust 
#[derive(Strung)]
struct Struct {
    name: &'static str,
    pos: u32,
}
fn main(){
    let s: String = Struct{name:"Bob", pos:10}.strung("{name} is {pos}th."); 
    // fields can also be addressed by their index
    let z: String = Struct{name:"Bob", pos:10}.strung("{0} is {1}th."); 
}
```
## Ignore 
🐳 use `#[igno]`, `#[ignore]`,`#[strung(igno)]` or `#[strung(ignore)]` to make a field unavailable<br> 
🦞 if a field type doesn't implement [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), it has to be ignored!
```rust 
struct NoDisplay;
#[derive(Strung)]
struct Struct (&'static str, u32, #[igno] NoDisplay);
fn main(){
    let s: String = Struct("Bob", 10, NoDisplay)
        .strung("{0} is {1}th, he won {2}!"); 
}
```
## Cascade
🐳 use `#[cscd]`, `#[cascade]`, `#[strung(cscd)]` or `#[strung(cascade)]` to cascade (recursion).<br>
🐑 cascaded fields are ignored by default <br>
🐔 use `#[notice]`, `#[ntce]` or inside `#[strung(..)]` to make them available, 
```rust 
#[derive(Strung)]
struct Struct &'static str, u32);
#[derive(Strung)]
struct Cascade (u32, #[cscd] Struct);
fn main(){
    let s: String = Cascade(11,Struct("Bob", 10))
        .strung("{1.0} is {1.1}th for the {0}th time!"); 
    // => Bob is 10th for the 11th time!
}
```
## Prefix and Postix Prefabs
🐈 5 different prefabs are provided:
```rust 
#[derive(Strung)]
struct Struct (&'static str, u32);
fn main(){
    let t = Struct("Bob", 10);
    let s = t.strung_curly("{0} is {1}th."); 
    let s = t.strung_angle("<0> is <1>th."); 
    let s = t.strung_dollry("${0} is ${1}th."); 
    let s = t.strung_dollar("$0 is $1th."); 
    let s = t.strung_hashtag("#0 is #1th."); 
}
```
## Custom Prefix and Postix
🦊 you can also customize pre-/postfixes in different ways<br>
🦅 **globally** - using static variables and `.strung_static(..)`:
```rust 
#[derive(Strung)]
struct Struct (&'static str, u32);
fn main(){
    strung::set_static("<",">");
    let s: String = Struct("Bob", 10).strung_static("<0> is <1>th."); 
}
```
🐎 **per struct** - this overrides the default `.strung(..)` pre/postfix:
```rust 
#[derive(Strung)]
#[strung("<",">")]
struct Struct (&'static str, u32);
fn main(){
    let s: String = Struct("Bob", 10).strung("<0> is <1>th."); 
}
```
🐍 **per call** - using parameters `.strung_dynamic(pre,post,..)`:
```rust 
#[derive(Strung)]
struct Struct (&'static str, u32);
fn main(){
    let s: String = Struct("Bob", 10).strung_dynamic("<",">","<0> is <1>th."); 
}
```
🦎 **per call** - using generic const chars `.strung_generic::<pre,post>(..)`:
```rust 
#[derive(Strung)]
struct Struct (&'static str, u32);
fn main(){
    let s: String = Struct("Bob", 10).strung_generic::<'<','>'>("<0> is <1>th."); 
}
```
## Performance Comparison
🐕 dynamic/generic/global have equal runtime speed<br>
🐇 default/prefabs/per-struct are faster!<br>
🐁 Using a string of ~650 chracters and 6 field placeholders:
<p align="center">
<img src="https://user-images.githubusercontent.com/78398528/282233001-d68aaac4-d419-44fa-bc3d-3ae62eb22fe4.svg">
</p>

## More Information
[🦕 Documentation](https://docs.rs/strung)<br>
<a href="CHANGELOG.md">🦎 Changelog</a><br>
[🐱 GitHub](https://github.com/dekirisu/strung)<br>
[👾 Discord Server](https://discord.gg/kevWvBuPFg)<br>

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>