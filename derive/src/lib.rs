/// Proc macro for strung!  
use proc_macro::TokenStream;
use quote::quote;

/// THE proc-macro, generating needed functions!
#[proc_macro_derive(Strung, attributes(strung,cascade,igno,cscd))]
pub fn strung_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_strung_macro(&ast)
}

fn impl_strung_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

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

        let mut idents       = (vec![], vec![]);
        let mut strs_main    = (vec![], vec![]);
        let mut strs_curly   = (vec![], vec![]);
        let mut strs_dollry  = (vec![], vec![]);
        let mut strs_angle   = (vec![], vec![]);
        let mut strs_dollar  = (vec![], vec![]);
        let mut strs_hashtag = (vec![], vec![]);
        let mut strs_raw     = (vec![], vec![]);

        let mut cscd_idents       = (vec![], vec![]);
        let mut cscd_strs_dollar  = (vec![], vec![]);
        let mut cscd_strs_hashtag = (vec![], vec![]);

        match &strct.fields {
            syn::Fields::Named(fields) => {
                for field in &fields.named {

                    let f_ident = field.ident.as_ref().unwrap().clone();
                    let f_name = field.ident.as_ref().unwrap().to_string();
                    
                    let mut ignore = false;
                    for aaa in &field.attrs {
                        let ewr = aaa.path.get_ident();
                        let nme = ewr.as_ref().unwrap().to_string();
                        /* ----------------------------- #[ignore/igno] ----------------------------- */
                        if &nme == "ignore" || &nme == "igno" {
                            ignore = true;
                        }
                        /* ----------------------------- #[cascade/cscd] ---------------------------- */
                        if &nme == "cascade" || &nme == "cscd" {
                            cscd_idents.0.push(f_ident.clone());
                            cscd_strs_dollar.0.push(format!("${}.",&f_name));
                            cscd_strs_hashtag.0.push(format!("#{}.",&f_name));
                            ignore = true;
                        }
                        /* ----------------------------- #[strung(...)] ----------------------------- */
                        else if &nme == "strung" {
                            if let Ok(bbb) = aaa.parse_meta() {
                                if let syn::Meta::List(list) = bbb {
                                    for abc in list.nested {
                                        if let syn::NestedMeta::Meta(meta) = abc {
                                            if let syn::Meta::Path(pp) = meta {
                                                let nident = pp.get_ident().as_ref().unwrap().to_string();
                                                if &nident == "cascade" || &nident == "cscd" {
                                                    cscd_idents.0.push(f_ident.clone());
                                                    cscd_strs_dollar.0.push(format!("${}.",&f_name));
                                                    cscd_strs_hashtag.0.push(format!("#{}.",&f_name));
                                                    ignore = true;
                                                } else if &nident == "ignore" || &nident == "igno" {
                                                    ignore = true;
                                                } else if &nident == "notice" || &nident == "notc" {
                                                    ignore = false;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        /* ----------------------------- #[notice/notc] ----------------------------- */
                        if &nme == "notice" || &nme == "notc" {
                            ignore = false;
                        }
                    }
                    if ignore {continue;}

                    idents.0.push(f_ident);
                    strs_main.0.push(format!("{}{}{}",&pre,&f_name,&post));

                    strs_curly.0.push(format!("{{{}}}",&f_name,));
                    strs_dollry.0.push(format!("${{{}}}",&f_name));
                    strs_angle.0.push(format!("<{}>",&f_name,));

                    strs_dollar.0.push(format!("${}",&f_name));
                    strs_hashtag.0.push(format!("#{}",&f_name));
                    
                    strs_raw.0.push(f_name.clone());
                }
            },
            syn::Fields::Unnamed(fields) => {
                let mut i = 0;
                for field in &fields.unnamed {
                    let mut ignore = false;
                    for aaa in &field.attrs {
                        let ewr = aaa.path.get_ident();
                        let nme = ewr.as_ref().unwrap().to_string();
                        /* ----------------------------- #[ignore/igno] ----------------------------- */
                        if &nme == "ignore" || &nme == "igno" {
                            ignore = true;
                        }
                        /* ----------------------------- #[cascade/cscd] ---------------------------- */
                        if &nme == "cascade" || &nme == "cscd" {
                            cscd_idents.1.push(syn::Index::from(i));
                            cscd_strs_dollar.1.push(format!("${}.",i));
                            cscd_strs_hashtag.1.push(format!("#{}.",i));
                            ignore = true;
                        }
                        /* ----------------------------- #[strung(...)] ----------------------------- */
                        if &nme == "strung" {
                            if let Ok(bbb) = aaa.parse_meta() {
                                if let syn::Meta::List(list) = bbb {
                                    for abc in list.nested {
                                        if let syn::NestedMeta::Meta(meta) = abc {
                                            if let syn::Meta::Path(pp) = meta {
                                                let nident = pp.get_ident().as_ref().unwrap().to_string();
                                                if &nident == "cascade" || &nident == "cscd" {
                                                    cscd_idents.1.push(syn::Index::from(i));
                                                    cscd_strs_dollar.1.push(format!("${}.",i));
                                                    cscd_strs_hashtag.1.push(format!("#{}.",i));
                                                    ignore = true;
                                                } else if &nident == "ignore" || &nident == "igno" {
                                                    ignore = true;
                                                } else if &nident == "notice" || &nident == "notc" {
                                                    ignore = false;
                                                }
                                            }
                                        }
                                    }
                                }
                            }                            
                        }
                        /* ----------------------------- #[notice/notc] ----------------------------- */
                        if &nme == "notice" || &nme == "notc" {
                            ignore = false;
                        }
                    }
                    if !ignore {

                        idents.1.push(syn::Index::from(i));
                        strs_main.1.push(format!("{}{}{}",&pre,i,&post));

                        strs_curly.1.push(format!("{{{}}}",i));
                        strs_dollry.1.push(format!("${{{}}}",i));
                        strs_angle.1.push(format!("<{}>",i));

                        strs_dollar.1.push(format!("${}",i));
                        strs_hashtag.1.push(format!("#{}",i));

                        strs_raw.1.push(i.to_string());
                    }
                    i += 1;
                }
            },
            _ => {},
        }

        macro_rules! prefab {
            ($name:ident, $idents:expr, $strs:expr) => {{ 
                let (id_nmd, id_unmd) = &$idents;
                let (strs_nmd, strs_unmd) = &$strs;
                quote! {
                fn $name(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#strs_nmd,  &self.#id_nmd.to_string());)*
                    #(output = output.replace(&#strs_unmd, &self.#id_unmd.to_string());)*
                    output
                }
            }}};
            ($litaf:expr, $name:ident, $idents:expr, $strs:expr, $cscd_idents:expr, $cscd_strs:expr) => {{ 
                let (id_nmd, id_unmd) = &$idents;
                let (strs_nmd, strs_unmd) = &$strs;
                let (cid_nmd, cid_unmd) = &$cscd_idents;
                let (cstrs_nmd, cstrs_unmd) = &$cscd_strs;
                quote! {
                fn $name(&self, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&#strs_nmd,  &self.#id_nmd.to_string());)*
                    #(output = output.replace(&#strs_unmd, &self.#id_unmd.to_string());)*
                    #(
                        output = output.replace(&#cstrs_nmd,$litaf);
                        output = self.#cid_nmd.$name(&output);
                    )*
                    #(
                        output = output.replace(&#cstrs_unmd,$litaf);
                        output = self.#cid_unmd.$name(&output);
                    )*
                    output
                }
            }}}
        }

        let strung          = prefab!(strung,        idents, strs_main   );
        let strung_curly    = prefab!(strung_curly,  idents, strs_curly  );
        let strung_angle    = prefab!(strung_angle,  idents, strs_angle  );
        let strung_dollry   = prefab!(strung_dollry, idents, strs_dollry );

        let strung_dollar   = prefab!("$",strung_dollar,  idents, strs_dollar,  cscd_idents, cscd_strs_dollar);
        let strung_hashtag  = prefab!("#",strung_hashtag, idents, strs_hashtag, cscd_idents, cscd_strs_hashtag);

        let (strs_raw_0, strs_raw_1) = strs_raw;
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
                    #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},&#strs_raw_0,unsafe{STRUNG_POST}),&self.#idents_0.to_string());)*
                    #(output = output.replace(&format!("{}{}{}",unsafe{STRUNG_PRE},&#strs_raw_1,unsafe{STRUNG_POST}),&self.#idents_1.to_string());)*
                    output
                }
                fn strung_dynamic(&self, pre: &str, post:&str, text: &str) -> String {
                    let mut output = text.to_string();
                    #(output = output.replace(&format!("{}{}{}",pre,&#strs_raw_0,post),&self.#idents_0.to_string());)*
                    #(output = output.replace(&format!("{}{}{}",pre,&#strs_raw_1,post),&self.#idents_1.to_string());)*
                    output
                }

            }
        };
        gen.into()
    } else {panic!("Not a Struct!")}
}