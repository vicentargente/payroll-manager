use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, Ident, ItemFn};

pub(crate) fn executor_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let original_sig = &input_fn.sig;
    let original_block = &input_fn.block;

    // Generate executor function name
    let executor_name = Ident::new(
        &format!("{}_executor", original_sig.ident),
        original_sig.ident.span(),
    );

    // Prepare executor function parameters
    let mut executor_inputs = original_sig.inputs.clone();
    let tx_param: FnArg = parse_quote!(tx: &mut SqliteConnection);

    // Insert tx parameter after self
    let insert_pos = if let Some(FnArg::Receiver(_)) = executor_inputs.first() {
        1
    } else {
        0
    };
    executor_inputs.insert(insert_pos, tx_param);

    // Build executor function signature
    let executor_sig = syn::Signature {
        ident: executor_name.clone(),
        inputs: executor_inputs,
        ..original_sig.clone()
    };

    // Generate the executor function
    let executor_fn = quote! {
        pub #executor_sig #original_block
    };

    // Prepare parameters for calling the executor, excluding self
    let call_params = original_sig.inputs.iter().enumerate().filter_map(|(i, arg)| {
        if i == 0 {
            if let FnArg::Receiver(_) = arg {
                None // Skip self
            } else {
                let arg_name = match arg {
                    FnArg::Typed(pat_type) => pat_type.pat.as_ref(),
                    _ => panic!("Unexpected function argument type"),
                };

                Some(quote!(#arg_name))
            }
        } else {
            let arg_name = match arg {
                FnArg::Typed(pat_type) => pat_type.pat.as_ref(),
                _ => panic!("Unexpected function argument type"),
            };

            Some(quote!(#arg_name))
        }
    });

    // Generate the new original function body
    let new_body = quote! {
        {
            let mut tx = self.db_pool.acquire().await.unwrap();
            self.#executor_name(&mut tx, #(#call_params),*).await
        }
    };

    // Original function with new body
    let new_original_fn = quote! {
        pub #original_sig #new_body
    };

    // Combine both functions
    let expanded = quote! {
        #new_original_fn
        #executor_fn
    };

    // println!("{}", expanded);

    TokenStream::from(expanded)
}
