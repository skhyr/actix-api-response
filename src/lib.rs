extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::{*, punctuated::Punctuated};



fn get_response_struct_declaration(ident: &Ident, fields: &FieldsNamed) -> ItemStruct {
    let new_ident = Ident::new(&format!("{}Response", ident), ident.span());
    ItemStruct {
        ident: new_ident,
        fields: Fields::Named(fields.clone()),
        vis: Visibility::Inherited,
        struct_token: Default::default(),
        generics: Default::default(),
        semi_token: Default::default(),
        attrs: vec![],
    }
}

fn construct_field_value(field: &Field) -> FieldValue{
    FieldValue {
            member: Member::Named(field.ident.clone().unwrap()),
            attrs: vec![],
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

fn get_response_struct(fields: &FieldsNamed) -> Expr {
    let field_values: Punctuated<FieldValue, Comma> = Punctuated::from_iter(fields.named.iter()
                        .map(|field| construct_field_value(field) ));
    Expr::Struct(ExprStruct {
        attrs: vec![],
        brace_token: syn::token::Brace::default(),
        fields: field_values,
        dot2_token: None,
        rest: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::new(),
        },
    })
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
    let filtered_fileds = fields;
    let response_struct_declaration = get_response_struct_declaration(&ident, &filtered_fileds);
    let response_struct = get_response_struct(&filtered_fileds);
    let temp = response_struct_declaration.ident.clone();

    quote! {
        impl Responder for #ident {
            type Body = actix_web::body::BoxBody;


            fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                #[derive(Serialize)]
                #response_struct_declaration

                let response_struct = #temp #response_struct;
                let body = serde_json::to_string(&response_struct).unwrap();
                actix_web::HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)
            }
        }
    }.into()
}
