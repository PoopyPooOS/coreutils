use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parse_macro_input, ExprParen, Ident, Token};

#[proc_macro]
pub fn coreutils(input: TokenStream) -> TokenStream {
    // Parse the input as a comma-separated list of commands
    let input = parse_macro_input!(input as CoreutilsInput);

    // Generate the match arms for each command
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

    // Generate the `mod` declarations for each command
    let module_declarations = input.commands.iter().map(|command| {
        let ident = &command.ident;
        quote! {
            mod #ident;
        }
    });

    // Generate the entire `main` function with the match statement and module declarations
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
                    eprintln!("This binary is used through a symlink.");
                    process::exit(1);
                }
                _ => {
                    eprintln!("Command not found: {exe}");
                    process::exit(1);
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// A struct to represent the parsed input of the coreutils! macro
struct CoreutilsInput {
    commands: Vec<Command>,
}

// A struct to represent a single command and whether it's a no-argument command
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

            // Allow for a comma or end of input
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
