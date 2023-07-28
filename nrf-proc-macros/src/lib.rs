use proc_macro::TokenStream;

use quote::quote;
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    LitInt, Token, Type,
};

mod keyword {
    syn::custom_keyword!(at);
    syn::custom_keyword!(by);
    syn::custom_keyword!(wrapped_in);
}

struct Args {
    enclosee: Type,
    enclosers: Punctuated<Type, Comma>,
    position: usize,
    wrapper: Type,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse enclosee
        let enclosee: Type = input.parse()?;

        // Parse position
        let _ = input.parse::<keyword::at>()?;
        let position_lit = input.parse::<LitInt>()?;
        let position = position_lit.base10_parse::<usize>()?;

        // Parse enclosers
        let _ = input.parse::<keyword::by>()?;

        let mut enclosers: Punctuated<Type, Comma> = Punctuated::new();
        enclosers.push_value(input.parse()?);

        while input.peek(Token![,]) {
            enclosers.push_punct(input.parse()?);
            enclosers.push_value(input.parse()?);
        }

        if position > enclosers.len() {
            return Err(Error::new(
                position_lit.span(),
                "position exceeds number of enclosing types",
            ));
        }

        // Parse wrapper
        let _ = input.parse::<keyword::wrapped_in>()?;

        let wrapper: Type = input.parse()?;

        Ok(Self {
            enclosee,
            enclosers,
            position,
            wrapper,
        })
    }
}

#[proc_macro]
pub fn enclose(tokens: TokenStream) -> TokenStream {
    let Args {
        enclosee,
        enclosers,
        position,
        wrapper,
    } = parse_macro_input!(tokens as Args);

    let before = enclosers.clone().into_iter().take(position);
    let after = enclosers.into_iter().skip(position);

    let comma: Option<Token![,]> = if position == 0 {
        None
    } else {
        Some(<Token![,]>::default())
    };

    let result: TokenStream = quote! {
        #wrapper<
            #(#before),* #comma
            #enclosee,
            #(#after),*
        >
    }
    .into();

    result
}
