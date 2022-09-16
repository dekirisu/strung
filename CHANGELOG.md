# v0.1.3 (2022-09-16)
* 🌟 derive: add `#[cascade]`/`#[cscd]` and `#[igno]` as alternatives to the more specific, but longer `#[strung(cascade)]` and `#[strung(ignore)]`, also add `#[strung(igno)]`
* 🎇 derive: cascade now auto-ignores, cause it's common to use it in combination
* 🌟 derive: add `#[notice]`/`#[notc]` and `#[strung(notice)]`/`#[strung(notc)]`, to override cascades auto-ignore
* 🧹 derive: clean up a lil
* 🐛 derive: fix macro to be re-exportable
* 📝 changelog: change wording

# v0.1.2 (2022-09-14)
* 🐛 derive: fix to work with structs with generic type parameters

# v0.1.1 (2022-09-12)
* 🌟 lib,derive: add `Strung::strung_angle(..)`: replaces angled bracets in strings: `"<field_name>"`
* 🌟 lib: add `StrungUnit`: empty struct with the `Strung` trait
* 📝 doc,readme,meta: minor fixes

# v0.1.0 (2022-09-12)
* ✨ first version: easy access to field via crates **strung** `./` and **strung_derive** `./derive`.