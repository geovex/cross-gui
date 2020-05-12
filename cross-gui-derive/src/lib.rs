extern crate proc_macro;
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn gui_trait(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::ItemTrait);
    let id = input.ident.clone();
    input.items.push(syn::parse_quote! {
        fn cloned(&mut self) -> Box<dyn #id>;
    });
    TokenStream::from(quote::quote!(#input))
}

#[proc_macro_attribute]
pub fn auto_clone(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut i = parse_macro_input!(input as syn::ItemImpl);
    let impl_target = i.trait_.clone();
    if impl_target.is_none() {
        return TokenStream::from(
            syn::Error::new(i.span(), "impl must imlement some type").to_compile_error(),
        );
    }
    let impl_target = impl_target.unwrap().1;
    i.items.push(syn::parse_quote! {
        fn cloned(&mut self) -> Box<dyn #impl_target> {
            Box::new(self.clone())
        }
    });
    let t = TokenStream::from(quote::quote!(#i));
    t
}
