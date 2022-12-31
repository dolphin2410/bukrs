use proc_macro::TokenStream;
use proc_macro2::TokenStream as NextStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, DataStruct, Data, Fields, FieldsNamed};

#[proc_macro_derive(BukrsPacket)]
pub fn bukrs_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }) = &input.data {
        named
    } else {
        panic!("BukrsPacket can only be derived for structs");
    };
    let ident = &input.ident;
    let encode_methods = fields.iter().map(|field| {
        let fident = field.ident.as_ref().unwrap();
        quote! {
            self.#fident.encode(target);
        }
    });

    let decode = fields.iter().map(|field| {
        let ty = &field.ty;
        quote! {
            #ty::decode(src)
        }
    });

    let decode_to_struct = (0..fields.len()).map(|i|{
        let field = &fields[i];
        let fident = field.ident.as_ref().unwrap();
        let decode = &decode.clone().collect::<Vec<NextStream>>()[i];
        quote! {
            #fident: #decode
        }
    });

    let expanded = quote! {
        impl bukrs_core::BukrsPacket for #ident {
            fn id(&self) -> String {
                stringify!(#ident).to_string()
            }

            fn encode(&self, target: &mut bytes::BytesMut) {
                #(#encode_methods)*
            }
        }

        impl bukrs_core::BukrsDecodable for #ident {
            fn decode(src: &mut bytes::BytesMut) -> #ident {
                #ident {
                    #(#decode_to_struct),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}