extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

/// Macro to mark a struct as a schema to be used with `meilimelo`
///
/// Right now, this macro only adds the same struct as child struct in a new
/// `_formatted` field. MeiliSearch uses this field to provide augmented data
/// in the results (highlights, crops, etc.).
///
/// # Example
///
/// ```
/// #[meilimelo::schema]
/// struct Employee {
///   firstname: String,
///   lastname: String
/// }
/// ```
///
/// The above struct renders as follows:
///
/// ```
/// struct FormattedEmployee {
///   firstname: String,
///   lastname: String
/// }
///
/// struct Employee {
///   firstname: String,
///   lastname: String,
///   #[serde(rename = "_formatted")]
///   formatted: FormattedEmployee
/// }
/// ```
#[proc_macro_attribute]
pub fn schema(_attribute: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as ItemStruct);
  let name = &input.ident;

  let fields = input.fields.iter().map(|field| {
    quote! {
      #field,
    }
  });

  let formatted_name = format_ident!("Formatted{}", name);
  let formatted_fields = fields.clone();

  let output = quote! {
    #[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
    struct #name {
      #(
        #fields
      )*
      #[serde(rename = "_formatted")]
      formatted: Option<#formatted_name>,
    }

    #[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
    struct #formatted_name {
      #(
        #formatted_fields
      )*
    }

    impl meilimelo::Schema for #name {}
  };

  TokenStream::from(output)
}
