use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn, Visibility, ReturnType, Type, parse};

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    // check the function signature
    let valid_signature = func.sig.constness.is_none()
        && func.vis == Visibility::Inherited
        && func.sig.abi.is_none()
        && func.sig.inputs.is_empty()
        && func.sig.generics.params.is_empty()
        && func.sig.generics.where_clause.is_none()
        && func.sig.variadic.is_none()
        && match func.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_))
        };

    if !valid_signature {
        return parse::Error::new(
            func.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    let tramp_ident = Ident::new(&format!("{}_trampoline", func.sig.ident), Span::call_site());
    let ident = &func.sig.ident;

    let expanded = quote!(
        #[doc(hidden)]
        #[export_name = "main"]
        pub unsafe extern "C" fn #tramp_ident() {
            #ident()
        }

        #func
    );
    
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    if args.is_empty() {
        return parse::Error::new(Span::call_site(), "This please provide exception name.")
            .to_compile_error()
            .into();
    }

    let args = args.to_string().replace("\"", "");

    let exception_name = match args.as_str() {
        "HardFault" | "NMI" | "SVCall" | "PendSV" | "SysTick" => args,
        _ => {
            return parse::Error::new(Span::call_site(), "Exception name only accepts: NMI, HardFault, SVCall, PendSV, SysTick.")
                .to_compile_error()
                .into();
        }
        
    };

    // check the function signature
    let valid_signature = func.sig.constness.is_none()
        && func.vis == Visibility::Inherited
        && func.sig.abi.is_none()
        && func.sig.inputs.is_empty()
        && func.sig.generics.params.is_empty()
        && func.sig.generics.where_clause.is_none()
        && func.sig.variadic.is_none()
        && match func.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_))
        };

    if !valid_signature {
        return parse::Error::new(
            func.span(),
            "`#[exception]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    let tramp_ident = Ident::new(&format!("{}_trampoline", func.sig.ident), Span::call_site());

    let func_block = func.block;

    let expanded = quote!(
        #[doc(hidden)]
        #[inline]
        #[export_name = #exception_name]
        pub unsafe extern "C" fn #tramp_ident()
            #func_block
    );
    
    TokenStream::from(expanded)
}
