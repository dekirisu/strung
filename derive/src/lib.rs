/// Proc macro for strung!  
use proc_macro::TokenStream;
use quote::quote;
use syn::{*, punctuated::Punctuated};

/* ---------------------------------- Core ---------------------------------- */

    trait IntuplePath {
        fn get_option (&self) -> Option<&'static str>;
    }
    impl IntuplePath for Path {
        fn get_option (&self) -> Option<&'static str> {
            if self.is_ident("ignore") || self.is_ident("igno") {
                Some("ignore")
            } else if self.is_ident("cascade") || self.is_ident("cscd") {
                Some("cascade")
            } else {None}
        }
    }

    trait IntupleAttributes {
        fn as_strings(&self) -> Vec<&'static str>;
    }
    impl IntupleAttributes for Vec<Attribute> {
        fn as_strings(&self) -> Vec<&'static str> {
            let mut names = vec![];
            for attr in self {
                if let Some(path) = attr.meta.path().get_option() {
                    names.push(path);
                }
                if attr.meta.path().is_ident("strung") {
                    attr.parse_nested_meta(|meta|{
                        if let Some(path) = meta.path.get_option() {
                            names.push(path);
                        }
                        Ok(())
                    }).unwrap();
                }
            }
            names
        }
    } 

    trait IntupleField {
        fn cascade(&self) -> bool;
        fn ignored(&self) -> bool;
    }
    impl IntupleField for Field {
        fn cascade(&self) -> bool {
            self.attrs.as_strings().contains(&"cascade")
        }
        fn ignored(&self) -> bool {
            self.attrs.as_strings().contains(&"ignore")
        }
    }

/* --------------------------------- Derive --------------------------------- */

/// THE proc-macro, generating needed functions!
#[proc_macro_derive(Strung, attributes(strung,cascade,igno,cscd))]
pub fn strung_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse(input).unwrap();
    impl_strung_macro(&ast)
}

fn impl_strung_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let (mut pre, mut post) = ("{".to_string(),"}".to_string());
    for attr in &ast.attrs {
        let ewr = attr.path().get_ident();
        if &ewr.as_ref().unwrap().to_string() == "strung" {
            if let Ok(list) = attr.meta.require_list(){
                let mut i = 0;
                let lit = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated).unwrap();
                for value in lit {
                    if i==0 {pre = value.value();}
                    else {post = value.value();}
                    i += 1;
                    if i==2 {continue;}
                }
            }
        }
    }

    if let Data::Struct(strct) = &ast.data {

        let mut idents       = (vec![], vec![]);
        let mut strs_main    = vec![];
        let mut strs_curly   = vec![];
        let mut strs_dollry  = vec![];
        let mut strs_angle   = vec![];
        let mut strs_dollar  = vec![];
        let mut strs_hashtag = vec![];
        let mut strs_raw     = vec![];

        let mut cscd_idents       = (vec![], vec![]);
        let mut cscd_strs_dollar  = vec![];
        let mut cscd_strs_hashtag = vec![];

        match &strct.fields {
            Fields::Named(fields) => {
                for field in &fields.named {

                    let f_ident = field.ident.as_ref().unwrap().clone();
                    let f_name = field.ident.as_ref().unwrap().to_string();
                    
                    if field.cascade() {
                        cscd_idents.0.push(f_ident.clone());
                        cscd_strs_dollar.push(format!("${}.",&f_name));
                        cscd_strs_hashtag.push(format!("#{}.",&f_name));
                        continue;
                    }
                    if field.ignored() {continue;}

                    idents.0.push(f_ident);
                    strs_main.push(format!("{}{}{}",&pre,&f_name,&post));

                    strs_curly.push(format!("{{{}}}",&f_name,));
                    strs_dollry.push(format!("${{{}}}",&f_name));
                    strs_angle.push(format!("<{}>",&f_name,));

                    strs_dollar.push(format!("${}",&f_name));
                    strs_hashtag.push(format!("#{}",&f_name));
                    
                    strs_raw.push(f_name.clone());
                }
            },
            Fields::Unnamed(fields) => {
                for (i,field) in fields.unnamed.iter().enumerate() {

                    if field.cascade() {
                        cscd_idents.1.push(Index::from(i));
                        cscd_strs_dollar.push(format!("${}.",i));
                        cscd_strs_hashtag.push(format!("#{}.",i));
                        continue;
                    }
                    if field.ignored() {continue;}

                    idents.1.push(Index::from(i));
                    strs_main.push(format!("{}{}{}",&pre,i,&post));

                    strs_curly.push(format!("{{{}}}",i));
                    strs_dollry.push(format!("${{{}}}",i));
                    strs_angle.push(format!("<{}>",i));

                    strs_dollar.push(format!("${}",i));
                    strs_hashtag.push(format!("#{}",i));

                    strs_raw.push(i.to_string());
                }
            },
            _ => {},
        }

        macro_rules! prefab {
            ($name:ident, $strs:expr) => {{ 
                let strs = $strs;
                let (id_nmd, id_unmd) = &idents;
                quote! {
                fn $name(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#strs,  &self.#id_nmd.to_string());)*
                    #(output = output.replace(&#strs, &self.#id_unmd.to_string());)*
                    output
                }
            }}};
            ($litaf:expr, $name:ident, $strs:expr, $cscd_strs:expr) => {{ 
                let strs = $strs;
                let (id_nmd, id_unmd) = &idents;
                let (cid_nmd, cid_unmd) = &cscd_idents;
                let cstrs = &$cscd_strs;
                quote! {
                fn $name(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#strs,  &self.#id_nmd.to_string());)*
                    #(output = output.replace(&#strs, &self.#id_unmd.to_string());)*
                    #(output = output.replace(&#cstrs,$litaf);)*
                    #(output = self.#cid_nmd.$name(&output);)*
                    #(output = self.#cid_unmd.$name(&output);)*
                    output
                }
            }}}
        }

        let strung          = prefab!(strung,        strs_main   );
        let strung_curly    = prefab!(strung_curly,  strs_curly  );
        let strung_angle    = prefab!(strung_angle,  strs_angle  );
        let strung_dollry   = prefab!(strung_dollry, strs_dollry );

        let strung_dollar   = prefab!("$",strung_dollar,  strs_dollar,  cscd_strs_dollar);
        let strung_hashtag  = prefab!("#",strung_hashtag, strs_hashtag, cscd_strs_hashtag);

        let (idents_0, idents_1) = idents;

        let gen = quote! {
            impl #impl_generics Strung for #name #ty_generics #where_clause {

                #strung
                #strung_curly
                #strung_angle
                #strung_dollry

                #strung_dollar
                #strung_hashtag

                fn strung_static(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},&#strs_raw,unsafe{STRUNG_POST}),&self.#idents_0.to_string());)*
                    #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},&#strs_raw,unsafe{STRUNG_POST}),&self.#idents_1.to_string());)*
                    output
                }
                fn strung_dynamic(&self, pre: &str, post:&str, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&format!("{}{}{}",pre,&#strs_raw,post),&self.#idents_0.to_string());)*
                    #(output = output.replace(&format!("{}{}{}",pre,&#strs_raw,post),&self.#idents_1.to_string());)*
                    output
                }

            }
        };
        gen.into()
    } else {panic!("Not a Struct!")}
}