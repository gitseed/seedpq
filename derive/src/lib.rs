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

    let struct_names_quoted: Vec<String> = data
        .fields
        .iter()
        .map(|x| x.ident.clone().unwrap().to_string())
        .collect();
    let struct_names_unquoted: Vec<Ident> = data
        .fields
        .iter()
        .map(|x| x.ident.clone().unwrap())
        .collect();
    let struct_types: Vec<syn::Type> = data.fields.iter().map(|x| x.ty.clone()).collect();

    let column_count: Ident = Ident::new(
        &format!("U{}", data.fields.len()),
        proc_macro2::Span::call_site(),
    );

    let mut try_from_blocks: Vec<proc_macro2::TokenStream> = Vec::new();

    for column in 0..data.fields.len() {
        try_from_blocks.push(try_from_block(
            &struct_names_quoted[column],
            &struct_names_unquoted[column],
            &struct_types[column],
            column,
        ))
    }

    let generated: proc_macro2::TokenStream = quote! {
        #[automatically_derived]
        impl ::seedpq::QueryResult<'_> for #name {
            type Columns = ::seedpq::hybrid_array::typenum::#column_count;
             
            const COLUMN_NAMES: ::std::option::Option<::seedpq::hybrid_array::Array<&'static str, Self::Columns>> = ::core::option::Option::Some(::seedpq::hybrid_array::Array([#(#struct_names_quoted),*]));
        }

        impl ::std::convert::TryFrom<::seedpq::hybrid_array::Array<::seedpq::PostgresData<'_>, ::seedpq::hybrid_array::typenum::#column_count>> for #name {
            type Error = ::seedpq::QueryResultError;

            fn try_from(data: ::seedpq::hybrid_array::Array<::seedpq::PostgresData, ::seedpq::hybrid_array::typenum::#column_count>) -> ::std::result::Result<Self, Self::Error> {
                #(#try_from_blocks)*;

                ::std::result::Result::Ok(#name { #(#struct_names_unquoted),* })
            }
        }
    };
    generated.into()
}

fn try_from_block(
    quoted_name: &String,
    unquoted_name: &Ident,
    ty: &syn::Type,
    column: usize,
) -> proc_macro2::TokenStream {
    quote! {
        let #unquoted_name: #ty = match ::std::convert::TryInto::try_into(data.0[#column]) {
            ::std::result::Result::Ok(value) => ::std::result::Result::Ok(value),
            ::std::result::Result::Err(e) => ::std::result::Result::Err(::seedpq::QueryResultError {
                e,
                t: #quoted_name,
                column: #column,
            }),
        }?;
    }
}
