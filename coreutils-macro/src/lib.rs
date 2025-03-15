use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{ExprParen, Ident, Token, parse_macro_input};

#[proc_macro]
pub fn coreutils(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CoreutilsInput);

    let match_arms = input.commands.iter().map(|command| {
        let ident = &command.ident;
        if command.has_args {
            quote! {
                stringify!(#ident) => {
                    logger::set_app_name!(stringify!(#ident));
                    #ident::#ident(args)
                },
            }
        } else {
            quote! {
                stringify!(#ident) => {
                    logger::set_app_name!(stringify!(#ident));
                    #ident::#ident()
                },
            }
        }
    });

    let module_declarations = input.commands.iter().map(|command| {
        let ident = &command.ident;
        quote! {
            mod #ident;
        }
    });

    let expanded = quote! {
        #(
            #module_declarations
        )*

        use std::{env, io, process};

        fn main() -> io::Result<()> {
            logger::panic::set_panic_hook();

            let exe = env::args().next().expect("Failed to get executable name");
            let exe = exe
                .split('/')
                .last()
                .expect("Failed to get executable name");

            let args = env::args();

            match exe {
                #(
                    #match_arms
                )*

                "coreutils" => {
                    panic!("This binary is used through a symlink.");
                }
                _ => {
                    panic!("Command not found: {exe}");
                }
            }
        }
    };

    TokenStream::from(expanded)
}

struct CoreutilsInput {
    commands: Vec<Command>,
}

struct Command {
    ident: Ident,
    has_args: bool,
}

impl Parse for CoreutilsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut commands = Vec::new();

        while !input.is_empty() {
            let command = input.parse::<Command>()?;
            commands.push(command);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(CoreutilsInput { commands })
    }
}

impl Parse for Command {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        let has_args = !input.peek(Paren);

        if !has_args {
            input.parse::<ExprParen>()?;
        }

        Ok(Command { ident, has_args })
    }
}
