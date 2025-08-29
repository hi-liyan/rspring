use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Marks a struct as an AxumBoot application entry point
#[proc_macro_attribute]
pub fn axum_boot_application(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        #input
        
        impl #name {
            pub async fn run() -> axum_boot_core::Result<()> {
                axum_boot_core::AxumBootApplication::new().run().await
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Marks a struct as a component for dependency injection
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl axum_boot_core::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Marks a struct as a service component
#[proc_macro_derive(Service)]
pub fn service_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl axum_boot_core::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        
        impl axum_boot_core::Service for #name {}
    };
    
    TokenStream::from(expanded)
}

/// Marks a struct as a repository component
#[proc_macro_derive(Repository)]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl axum_boot_core::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        
        impl axum_boot_core::Repository for #name {}
    };
    
    TokenStream::from(expanded)
}

/// Marks a struct as a REST controller
#[proc_macro_derive(RestController)]
pub fn rest_controller_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl axum_boot_core::Component for #name {
            fn component_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        
        impl axum_boot_core::Controller for #name {}
    };
    
    TokenStream::from(expanded)
}