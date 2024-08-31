extern crate proc_macro;
use proc_macro::{TokenStream};

use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, Ident, ItemFn, Pat, Stmt};
use darling::{FromMeta, ToTokens};

#[derive(FromMeta)]
struct MacroArgs {
  #[darling(default)]
  verbose: bool,
}

// Macro attribute entry function
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
  let attr_args_list = parse_macro_input!(args as AttributeArgs);
  let mut input = parse_macro_input!(input as ItemFn);

  let attr_args = match MacroArgs::from_list(&attr_args_list) {
      Ok(a) => a,
      Err(e) => {
          return TokenStream::from(e.write_errors());
      }
  };
  impl_log_call(&attr_args, &mut input)
}

// Macro attribute definition function
fn impl_log_call(args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
  let fn_name = &input.sig.ident;
  if args.verbose {
    let arg_names = extract_args_names(input);
    let statements = generate_verbose_log(fn_name, arg_names);
    input.block.stmts.splice(0..0, statements);
  } else {
    input.block.stmts.insert(0, parse_quote!(
      println!("[Info] calling '{}' function", stringify!(#fn_name));
    ));
  }
  input.to_token_stream().into() 
}

fn extract_args_names(func: &ItemFn) -> Vec<&Ident> {
  func.sig.inputs.iter().filter_map(|arg| {
    if let FnArg::Typed(pat_type) = arg {
      if let Pat::Ident(pat) = &(*pat_type.pat) {
        return Some(&pat.ident)
      }
    }
    None
  }).collect()
}

fn generate_verbose_log(fn_name: &Ident, fn_args: Vec<&Ident>) -> Vec<Stmt>{
  let mut statements = vec![parse_quote!{
    print!("[Info] calling {} |", stringify!(#fn_name));
  }];

  for arg in fn_args {
    statements.push(parse_quote!{
      print!("{} = {:?}", stringify!(#arg), #arg);
    });
  }

  statements.push(parse_quote!{ println!(); });

  statements
}