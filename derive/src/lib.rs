/// Proc macro for strung!  

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// THE proc-macro, generating needed functions!
#[proc_macro_derive(Strung, attributes(strung))]
pub fn strung_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_strung_macro(&ast)
}

fn impl_strung_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (mut pre, mut post) = ("{".to_string(),"}".to_string());
    for aaa in &ast.attrs {
        let ewr = aaa.path.get_ident();
        if &ewr.as_ref().unwrap().to_string() == "strung" {
            if let Ok(bbb) = aaa.parse_meta() {
                if let syn::Meta::List(list) = bbb {
                    let mut i = 0;
                    for abc in list.nested {
                        if let  syn::NestedMeta::Lit(lit) = abc {
                            if let syn::Lit::Str(string) = lit {
                                if i==0 {pre = string.value();}
                                else {post = string.value();}
                            }
                        }
                        i += 1;
                        if i==2 {break;}
                    }
                }
            }
        }
    }
    if let syn::Data::Struct(strct) = &ast.data {
        
        let mut types = vec![];

        let mut idents = vec![];
        let mut fnames = vec![];
        let mut fnames_curly = vec![];
        let mut fnames_dollar = vec![];
        let mut fnames_dollry = vec![];
        let mut fnames_hashtag = vec![];
        let mut fnamesraw = vec![];

        let mut unn = vec![];
        let mut unn_curly = vec![];
        let mut unn_dollar = vec![];
        let mut unn_dollry = vec![];
        let mut unn_hashtag = vec![];
        let mut unnraw = vec![];
        let mut unnid = vec![];

        let mut cscd_nmd = vec![];
        let mut cscd_nmd_str = vec![];
        let mut cscd_nmd_str_h = vec![];
        let mut cscd_unmd = vec![];
        let mut cscd_unmd_str = vec![];
        let mut cscd_unmd_str_h = vec![];

        match &strct.fields {
            syn::Fields::Named(fields) => {
                for field in &fields.named {

                    let f_ident = field.ident.as_ref().unwrap().clone();
                    let f_name = field.ident.as_ref().unwrap().to_string();
                    
                    let mut ignore = false;
                    for aaa in &field.attrs {
                        let ewr = aaa.path.get_ident();
                        let nme = ewr.as_ref().unwrap().to_string();
                        if &nme == "strung" {
                            if let Ok(bbb) = aaa.parse_meta() {
                                if let syn::Meta::List(list) = bbb {
                                    for abc in list.nested {
                                        if let syn::NestedMeta::Meta(meta) = abc {
                                            if let syn::Meta::Path(pp) = meta {
                                                let nident = pp.get_ident().as_ref().unwrap().to_string();
                                                if &nident == "cascade" {
                                                    cscd_nmd.push(f_ident.clone());
                                                    cscd_nmd_str.push(format!("${}.",&f_name));
                                                    cscd_nmd_str_h.push(format!("#{}.",&f_name));
                                                } else if &nident == "ignore" {
                                                    ignore = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if ignore {continue;}

                    types.push(field.ty.clone());
                    idents.push(f_ident);
                    fnamesraw.push(f_name.clone());
                    fnames.push(format!("{}{}{}",&pre,&f_name,&post));
                    fnames_curly.push(format!("{{{}}}",&f_name,));
                    fnames_dollar.push(format!("${}",&f_name));
                    fnames_dollry.push(format!("${{{}}}",&f_name));
                    fnames_hashtag.push(format!("#{}",&f_name));
                }
            },
            syn::Fields::Unnamed(fields) => {
                let mut i = 0;
                for field in &fields.unnamed {
                    let mut ignore = false;
                    for aaa in &field.attrs {
                        let ewr = aaa.path.get_ident();
                        let nme = ewr.as_ref().unwrap().to_string();
                        if &nme == "strung" {
                            if let Ok(bbb) = aaa.parse_meta() {
                                if let syn::Meta::List(list) = bbb {
                                    for abc in list.nested {
                                        if let syn::NestedMeta::Meta(meta) = abc {
                                            if let syn::Meta::Path(pp) = meta {
                                                let nident = pp.get_ident().as_ref().unwrap().to_string();
                                                if &nident == "cascade" {
                                                    cscd_unmd.push(syn::Index::from(i));
                                                    cscd_unmd_str.push(format!("${}.",i));
                                                    cscd_unmd_str_h.push(format!("#{}.",i));
                                                } else if &nident == "ignore" {
                                                    ignore = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }                            
                        }
                    }
                    if !ignore {
                        types.push(field.ty.clone());

                        unn.push(format!("{}{}{}",&pre,i,&post));
                        unnraw.push(i.to_string());
                        unnid.push(syn::Index::from(i));
                        
                        unn_curly.push(format!("{{{}}}",i));
                        unn_dollar.push(format!("${}",i));
                        unn_dollry.push(format!("${{{}}}",i));
                        unn_hashtag.push(format!("#{}",i));
                    }
                    i += 1;
                }
            },
            _ => {},
        }
        let gen = quote! {
            impl Strung for #name {
                fn strung(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#fnames,&self.#idents.to_string());)*
                    #(output = output.replace(&#unn,&self.#unnid.to_string());)*
                    output
                }
                fn strung_static(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},&#fnamesraw,unsafe{STRUNG_POST}),&self.#idents.to_string());)*
                    #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},&#unnraw,unsafe{STRUNG_POST}),&self.#unnid.to_string());)*
                    output
                }
                fn strung_dynamic(&self, pre: &str, post:&str, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&format!("{}{}{}",pre,&#fnamesraw,post),&self.#idents.to_string());)*
                    #(output = output.replace(&format!("{}{}{}",pre,&#unnraw,post),&self.#unnid.to_string());)*
                    output
                }

                fn strung_curly(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#fnames_curly,&self.#idents.to_string());)*
                    #(output = output.replace(&#unn_curly,&self.#unnid.to_string());)*
                    output
                }
                fn strung_dollar(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#fnames_dollar,&self.#idents.to_string());)*
                    #(output = output.replace(&#unn_dollar,&self.#unnid.to_string());)*
                    #(
                        output = output.replace(&#cscd_nmd_str,"$");
                        output = self.#cscd_nmd.strung_dollar(&output);
                    )*
                    #(
                        output = output.replace(&#cscd_unmd_str,"$");
                        output = self.#cscd_unmd.strung_dollar(&output);
                    )*
                    output
                }
                fn strung_dollry(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#fnames_dollry,&self.#idents.to_string());)*
                    #(output = output.replace(&#unn_dollry,&self.#unnid.to_string());)*
                    output
                }
                fn strung_hashtag(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#fnames_hashtag,&self.#idents.to_string());)*
                    #(output = output.replace(&#unn_hashtag,&self.#unnid.to_string());)*
                    #(
                        output = output.replace(&#cscd_nmd_str_h,"#");
                        output = self.#cscd_nmd.strung_hashtag(&output);
                    )*
                    #(
                        output = output.replace(&#cscd_unmd_str_h,"#");
                        output = self.#cscd_unmd.strung_hashtag(&output);
                    )*
                    output
                }

            }
        };
        gen.into()
    } else {panic!("Not a Struct!")}
}