extern crate syn;
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::*;

fn construct_field_value(field: &Field) -> FieldValue{
    FieldValue {
            member: Member::Named(field.ident.clone().unwrap()),
            attrs: field.attrs.clone(),
            expr: Expr::Field(ExprField {
                attrs: vec![],
                base: Box::new(Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: syn::Path::from(syn::Ident::new("self", field.ident.clone().unwrap().span() )),
                })),
                dot_token: syn::token::Dot::default(),
                member: Member::Named(field.ident.clone().unwrap())
            }),
            colon_token: Some(syn::token::Colon::default()),
    }
}

#[proc_macro_derive(ApiResponse, attributes(apiResponseSkip))]
pub fn api_response_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let struct_data = match data {
        Data::Struct(d) => d,
        Data::Enum(_) => panic!("Enum are not supported"),
        Data::Union(_) => panic!("Unions are not supported"),
    };
    let fields = match struct_data.fields {
        Fields::Named(f) => f,
        _ => panic!("Only named fields are supported"),
    };
    // TODO Omit unwanted fields
    // attr.path.is_ident("apiResponseSkip")
    let response_ident = Ident::new(&format!("{}Response", ident), ident.span());
    let filtered_fileds = fields.named.iter();
    let field_values = fields.named.iter().map(|f| construct_field_value(f));

    quote! {
        impl Responder for #ident {
            type Body = actix_web::body::BoxBody;

            fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                #[derive(Serialize)]
                struct #response_ident {
                    #(#filtered_fileds.iter()),*
                }
                let response_object = #response_ident {
                    #(#field_values),*
                };
                let body = serde_json::to_string(&response_struct).unwrap();
                actix_web::HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)
            }
        }
    }.into()
}
