/// Proc macro for strung!  
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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
            } else if self.is_ident("notice") || self.is_ident("notc") {
                Some("notice")
            }else {None}
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
        fn notice(&self) -> bool;
    }
    impl IntupleField for Field {
        fn cascade(&self) -> bool {
            self.attrs.as_strings().contains(&"cascade")
        }
        fn ignored(&self) -> bool {
            self.attrs.as_strings().contains(&"ignore")
        }
        fn notice(&self) -> bool {
            self.attrs.as_strings().contains(&"notice")
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

    match &ast.data {
        Data::Struct(strct) => {

            let mut idents       = vec![];
            let mut posids       = vec![];
            let mut strs_main    = vec![];
            let mut strs_curly   = vec![];
            let mut strs_dollry  = vec![];
            let mut strs_angle   = vec![];
            let mut strs_dollar  = vec![];
            let mut strs_hashtag = vec![];
    
            let mut cscd_idents       = vec![];
            let mut cscd_posids       = vec![];
            let mut cscd_strs_main    = vec![];
            let mut cscd_strs_curly   = vec![];
            let mut cscd_strs_dollry  = vec![];
            let mut cscd_strs_angle   = vec![];
            let mut cscd_strs_dollar  = vec![];
            let mut cscd_strs_hashtag = vec![];
    
            for (position,field) in strct.fields.iter().enumerate() {
                
                let ident = match &field.ident {
                    Some(id) => id.to_token_stream(),
                    None => Index::from(position).to_token_stream(),
                };
                let posid = Index::from(position).to_token_stream();

                if field.cascade() {
                    cscd_idents.push(ident.clone());
                    cscd_strs_main.push(format!("{}{}.",&pre,&ident));
                    cscd_strs_curly.push(format!("{{{}.",&ident));
                    cscd_strs_dollry.push(format!("${{{}.",&ident));
                    cscd_strs_angle.push(format!("<{}.",&ident));
                    cscd_strs_dollar.push(format!("${}.",&ident));
                    cscd_strs_hashtag.push(format!("#{}.",&ident));
                    if field.ident.is_some() {
                        cscd_idents.push(ident.clone());
                        cscd_posids.push(posid.clone());
                        cscd_strs_main.push(format!("{}{}.",&pre,&posid));
                        cscd_strs_curly.push(format!("{{{}.",&posid));
                        cscd_strs_dollry.push(format!("${{{}.",&posid));
                        cscd_strs_angle.push(format!("<{}.",&posid));
                        cscd_strs_dollar.push(format!("${}.",&posid));
                        cscd_strs_hashtag.push(format!("#{}.",&posid));
                    }
                }
                if field.ignored() || field.cascade() && !field.notice() {continue;}
    
                idents.push(ident.clone());
                posids.push(posid.clone());
                strs_main.push(format!("{}{}{}",&pre,&ident,&post));
                strs_curly.push(format!("{{{}}}",&ident,));
                strs_dollry.push(format!("${{{}}}",&ident));
                strs_angle.push(format!("<{}>",&ident,));
                strs_dollar.push(format!("${}",&ident));
                strs_hashtag.push(format!("#{}",&ident));
                if field.ident.is_some() {
                    idents.push(ident);
                    strs_main.push(format!("{}{}{}",&pre,&posid,&post));
                    strs_curly.push(format!("{{{}}}",&posid,));
                    strs_dollry.push(format!("${{{}}}",&posid));
                    strs_angle.push(format!("<{}>",&posid,));
                    strs_dollar.push(format!("${}",&posid));
                    strs_hashtag.push(format!("#{}",&posid));
                }

            }
    
            macro_rules! prefab {
                ($name:ident, $strs:expr) => {{ 
                    let strs = $strs;
                    quote! {
                    fn $name(&self, text: &str) -> String {
                        let mut output = text.to_string();
                        #(output = output.replace(&#strs, &self.#idents.to_string());)*
                        output
                    }
                }}};
                ($litaf:expr, $name:ident, $strs:expr, $cscd_strs:expr) => {{ 
                    let strs = $strs;
                    let cstrs = &$cscd_strs;
                    quote! {
                    fn $name(&self, text: &str) -> String {
                        let mut output = text.to_string();
                        #(output = output.replace(&#strs, &self.#idents.to_string());)*
                        #(output = output.replace(&#cstrs,$litaf);)*
                        #(output = self.#cscd_idents.$name(&output);)*
                        output
                    }
                }}}
            }
    
            let strung = quote! {
                fn strung(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#strs_main, &self.#idents.to_string());)*
                    #(output = output.replace(&#cscd_strs_main,#pre);)*
                    #(output = self.#cscd_idents.strung(&output);)*
                    output
                }
            };
            let strung_curly    = prefab!("{",strung_curly,   strs_curly,   cscd_strs_curly);
            let strung_angle    = prefab!("<",strung_angle,   strs_angle,   cscd_strs_angle);
            let strung_dollry   = prefab!("${",strung_dollry, strs_dollry,  cscd_strs_dollry);
            let strung_dollar   = prefab!("$",strung_dollar,  strs_dollar,  cscd_strs_dollar);
            let strung_hashtag  = prefab!("#",strung_hashtag, strs_hashtag, cscd_strs_hashtag);
    
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
                        #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},stringify!(#idents),unsafe{STRUNG_POST}),&self.#idents.to_string());)*
                        #(output = output.replace(&format!("{}{}.",unsafe{STRUNG_PRE},stringify!(#cscd_idents)),unsafe{STRUNG_PRE});)*
                        #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},stringify!(#posids),unsafe{STRUNG_POST}),&self.#idents.to_string());)*
                        #(output = output.replace(&format!("{}{}.",unsafe{STRUNG_PRE},stringify!(#cscd_posids)),unsafe{STRUNG_PRE});)*
                        #(output = self.#cscd_idents.strung_static(&output);)*
                        output
                    }
                    fn strung_dynamic(&self, pre: &str, post:&str, text: &str) -> String {
                        let mut output = text.to_string();
                        #(output = output.replace(&format!("{}{}{}",pre,stringify!(#idents),post),&self.#idents.to_string());)*
                        #(output = output.replace(&format!("{}{}.",pre,stringify!(#cscd_idents)),pre);)*
                        #(output = output.replace(&format!("{}{}{}",pre,stringify!(#posids),post),&self.#idents.to_string());)*
                        #(output = output.replace(&format!("{}{}.",pre,stringify!(#cscd_posids)),pre);)*
                        #(output = self.#cscd_idents.strung_dynamic(pre,post,&output);)*
                        output
                    }
    
                    fn strung_generic <const STRUNG_PRE:char,const STRUNG_POST:char> (&self, text: &str) -> String {
                        let pre = STRUNG_PRE.to_string();
                        let post = STRUNG_POST.to_string();
                        let mut output = text.to_string();
                        #(output = output.replace(&format!("{}{}{}",&pre,stringify!(#idents),post),&self.#idents.to_string());)*
                        #(output = output.replace(&format!("{}{}.",&pre,stringify!(#cscd_idents)),&pre);)*
                        #(output = output.replace(&format!("{}{}{}",&pre,stringify!(#posids),post),&self.#idents.to_string());)*
                        #(output = output.replace(&format!("{}{}.",&pre,stringify!(#cscd_posids)),&pre);)*
                        #(output = self.#cscd_idents.strung_dynamic(&pre,&post,&output);)*
                        output
                    }
    
                }
            };
            gen.into()
        },
        Data::Enum(_) => {todo!()},
        Data::Union(_) => panic!("Unions not supported!"),
    }
}