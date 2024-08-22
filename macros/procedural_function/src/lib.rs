extern crate proc_macro;

use chrono::Utc;
use quote::quote;
use proc_macro::TokenStream;

// Function type macro
#[proc_macro]
pub fn log_time(input: TokenStream) -> TokenStream {
  let mut output = "[Info] ".to_owned();

  // Recorremos la secuencia de token streams intercambiando [TIME] por la fecha
  for token in input {
    let token_string = token.to_string();
    match token_string.as_str() {
      "[TIME]" => {
        let time = Utc::now().time().to_string();
        output.push_str(&format!("{} ", time));
      }
      _ => {
        output.push_str(&format!("{} ", token_string))
      }
    }
  }

  // Devolvemos un token stream que usa el output
  TokenStream::from(quote! {
    println!("{}", #output)
  })
}
