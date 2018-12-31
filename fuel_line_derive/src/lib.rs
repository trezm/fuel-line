extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[proc_macro_derive(Render, attributes(TemplateName))]
pub fn render(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let mut template_location = None;
    for attr in &ast.attrs {
        match attr.interpret_meta().unwrap() {
            syn::Meta::NameValue(val) => {
                if val.ident.to_string() == "TemplateName" {
                    if let syn::Lit::Str(lit) = &val.lit {
                        template_location = Some(lit.value());
                    }
                }
            },
            _ => ()
        };
    }

    let template_location = template_location
        .expect("Could not find TemplateName attribute");

    let mut f = File::open(&template_location)
        .expect(&format!("Couldn't find the file: {}", &template_location));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Couldn't read the file: {}", &template_location));

    // Build the impl
    let gen = generate_tokens(&ast, &contents);

    // Return the generated impl
    gen.into()
}

fn generate_tokens(ast: &syn::DeriveInput, string_to_interpolate: &str) -> quote::Tokens {
    let name = &ast.ident;

    let mut ident_map = HashMap::new();
    if let syn::Data::Struct(data) = &ast.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            for field in fields.named.iter() {
                let field_ident = field.ident.unwrap();

                ident_map.insert(field_ident.to_string(), field_ident);
            }
        }
    }

    let mut length_quote = quote! {};
    let mut concat_quote = quote! {};
    for piece in string_to_interpolate.split("{{") {
        let close_tag = piece.find("}}");

        if let Some(index) = close_tag {
            let field_ident = ident_map.get(&piece[0..index].trim().to_owned())
                .expect(&format!("Could not locate field: {}", piece[0..index].to_owned()));
            let the_rest = piece[index + 2..].to_owned();
            length_quote.append_all(quote! {
                total_length = total_length + &self.#field_ident.len() + #the_rest.len();
            });

            concat_quote.append_all(quote! {
                output_string.push_str(&self.#field_ident);
                output_string.push_str(#the_rest);
            });
        } else {
            length_quote.append_all(quote! {
                total_length = total_length + #piece.len();
            });

            concat_quote.append_all(quote! {
                output_string.push_str(#piece);
            });
        }
    }

    let q = quote! {
        impl Render for #name {
            fn render(&self) -> String {
                let mut total_length = 0;

                #length_quote

                let mut output_string = String::with_capacity(total_length);

                #concat_quote

                output_string
            }
        }
    };

    q
}
