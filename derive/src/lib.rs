use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;

#[proc_macro_derive(QueryResult)]
pub fn query_result_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    impl_query_result_macro(&ast)
}

fn impl_query_result_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name: &syn::Ident = &ast.ident;
    let data: &syn::Data = &ast.data;

    let data = match data {
        syn::Data::Struct(expr) => expr,
        _ => panic!("QueryResult Derive is only implemented for structs"),
    };

    let column_count: Ident = Ident::new(
        &format!("U{}", data.fields.len()),
        proc_macro2::Span::call_site(),
    );

    let generated: proc_macro2::TokenStream = quote! {
        #[automatically_derived]
        impl seedpq::QueryResult<'_> for #name {
            type Columns = ::seedpq::hybrid_array::typenum::#column_count;
            const COLUMN_NAMES: seedpq::hybrid_array::Array<&'static str, Self::Columns> = seedpq::hybrid_array::Array(["version"]);
        }
    };
    generated.into()
}
