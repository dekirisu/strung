# v0.1.3 (2022-09-XX)
### Derive Macro `./derive`
* 🌟 add `#[cascade]`/`#[cscd]` and `#[igno]` as alternatives to the more specific, but longer `#[strung(cascade)]` and `#[strung(ignore)]`, also add `#[strung(igno)]`
* 🎇 cascade now auto-ignores, cause it's common to use it in combination
* 🌟 add `#[notice]`/`#[notc]` and `#[strung(notice)]`/`#[strung(notc)]`, to override cascades auto-ignore
* 🧹 clean up a lil
* 🐛 fix macro to be re-exportable
### Library `./`
* 💡 update doc to changes
### Readme
* 📝 remove license badge
* 📝 rewrite all
* 🦀 use animal emojis for fun!
### Other
* 📝 changelog: change wording & use titles
# v0.1.2 (2022-09-14)
### Derive Macro `./derive`
* 🐛 fix to work with structs with generic type parameters

# v0.1.1 (2022-09-12)
### Library `./` & Derive Macro `./derive`
* 🌟 add `Strung::strung_angle(..)`: replaces angled bracets in strings: `"<field_name>"`
### Derive Macro `./derive`
* 🌟 add `StrungUnit`: empty struct with the `Strung` trait
### Other
* 📝 doc,readme,meta: minor fixes

# v0.1.0 (2022-09-12)
* ✨ first version: easy access to field via crates **strung** `./` and **strung_derive** `./derive`.