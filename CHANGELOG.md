# v0.1.3 (2022-09-XX)
### Derive Macro `./derive`
* ๐ add `#[cascade]`/`#[cscd]` and `#[igno]` as alternatives to the more specific, but longer `#[strung(cascade)]` and `#[strung(ignore)]`, also add `#[strung(igno)]`
* ๐ cascade now auto-ignores, cause it's common to use it in combination
* ๐ add `#[notice]`/`#[notc]` and `#[strung(notice)]`/`#[strung(notc)]`, to override cascades auto-ignore
* ๐งน clean up a lil
* ๐ fix macro to be re-exportable
### Library `./`
* ๐ก update doc to changes
### Readme
* ๐ remove license badge
* ๐ rewrite all
* ๐ฆ use animal emojis for fun!
* ๐จ add animated header for fun!
### Other
* ๐ changelog: change wording & use titles
# v0.1.2 (2022-09-14)
### Derive Macro `./derive`
* ๐ fix to work with structs with generic type parameters

# v0.1.1 (2022-09-12)
### Library `./` & Derive Macro `./derive`
* ๐ add `Strung::strung_angle(..)`: replaces angled bracets in strings: `"<field_name>"`
### Derive Macro `./derive`
* ๐ add `StrungUnit`: empty struct with the `Strung` trait
### Other
* ๐ doc,readme,meta: minor fixes

# v0.1.0 (2022-09-12)
* โจ first version: easy access to field via crates **strung** `./` and **strung_derive** `./derive`.