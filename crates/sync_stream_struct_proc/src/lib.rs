use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

const STRUCT_NAME_PREFIX: &str = "SyncStream";

struct Idents {
    stream_name: Ident,
    last_name: Ident,
    pending_name: Ident,
    done_name: Ident,
    ty: Ident,
    stream_type: Ident,
}

struct Context<'a> {
    name: &'a Ident,
    generics: &'a TokenStream,
    where_clause: &'a TokenStream,
    idents: &'a [Idents],
    mixed_ty: &'a Ident,
}

fn gen_impl_new(
    Context {
        name,
        generics,
        where_clause,
        idents,
        ..
    }: &Context,
) -> TokenStream {
    let params = idents.iter().map(
        |Idents {
             stream_name,
             stream_type,
             ..
         }| { quote!(#stream_name: #stream_type) },
    );

    let values = idents.iter().flat_map(
        |Idents {
             stream_name,
             last_name,
             pending_name,
             done_name,
             ..
         }| {
            [
                quote!(#stream_name: #stream_name.fuse()),
                quote!(#last_name: None),
                quote!(#pending_name: None),
                quote!(#done_name: false),
            ]
        },
    );

    quote! {
        impl #generics #name #generics #where_clause {
            #[allow(clippy::too_many_arguments)]
            pub fn new(
                #(#params),*
            ) -> Self {
                Self {
                    #(#values),*
                }
            }
        }
    }
}

fn gen_impl_stream(
    Context {
        name,
        generics,
        where_clause,
        idents,
        mixed_ty,
        ..
    }: &Context,
) -> TokenStream {
    let assoc_items = idents
        .iter()
        .map(|Idents { stream_type, .. }| quote!(Option<#stream_type::Item>));

    let assoc_type = quote! {
        (#(#assoc_items),*)
    };

    let poll_items = idents.iter().map(
        |Idents {
             pending_name,
             stream_name,
             done_name,
             ..
         }| {
            quote!(
                //only fetch if don't have an item pending
                if !*this.#done_name && this.#pending_name.is_none() {
                    match this.#stream_name.poll_next(cx) {
                        Poll::Ready(Some(val)) => {
                            this.#pending_name.replace(val);
                        },
                        Poll::Ready(None) => {
                            *this.#done_name = true;
                        },
                        Poll::Pending => {},
                    }
                }
            )
        },
    );

    let items_ready = idents.iter().map(
        |Idents {
             pending_name,
             done_name,
             ..
         }| { quote!((this.#pending_name.is_some() || *this.#done_name)) },
    );

    let items_done = idents
        .iter()
        .map(|Idents { done_name, .. }| quote!(*this.#done_name));

    let pending_items = idents.iter().map(
        |Idents {
             pending_name, ty, ..
         }| { quote!(this.#pending_name.as_ref().map(mixed_array::#mixed_ty::#ty)) },
    );

    let last_items = idents
        .iter()
        .map(|Idents { last_name, .. }| quote!(this.#last_name.clone()));

    let match_replace_item = idents.iter().map(
        |Idents {
             pending_name,
             last_name,
             ty,
             ..
         }| {
            quote! {
                mixed_array::#mixed_ty::#ty(_) => {
                    if let Some(next_item) = this.#pending_name.take() {
                        this.#last_name.replace(next_item);
                    }
                }
            }
        },
    );

    quote! {
        impl #generics Stream for #name #generics #where_clause {
            type Item = #assoc_type;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                let this = self.project();

                #(#poll_items)*

                //move the smallest pending items into the current queue
                if #(#items_done)&&* {
                    Poll::Ready(None)
                } else if #(#items_ready)&&* {
                    //determine which next stream item is the smallest
                    //we could use a BinaryHeap here instead to keep track of the next item to process based on order
                    let next_to_process = [
                        #(#pending_items),*
                    ]
                        .into_iter()
                        .flatten()
                        .min();

                    if let Some(next_to_process) = &next_to_process {
                        match next_to_process {
                            #(#match_replace_item),*
                        }
                    }

                    Poll::Ready(Some((
                        #(#last_items),*
                    )))
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

fn gen_sync_struct(index: usize) -> TokenStream {
    let name = format_ident!("{STRUCT_NAME_PREFIX}{index}");

    let idents = ('A'..='Z')
        .take(index)
        .map(|char| {
            let item = char.to_lowercase();

            Idents {
                stream_name: format_ident!("stream_{item}"),
                last_name: format_ident!("last_{item}"),
                pending_name: format_ident!("pending_{item}"),
                done_name: format_ident!("done_{item}"),
                stream_type: format_ident!("Stream{char}"),
                ty: format_ident!("{char}"),
            }
        })
        .collect::<Vec<_>>();

    let partialord_all = idents
        .iter()
        .map(|Idents { stream_type, .. }| quote!(PartialOrd<#stream_type::Item>))
        .collect::<Vec<_>>();

    let where_conditions = idents.iter().flat_map(|Idents { stream_type, .. }| {
        [
            quote!(#stream_type: Stream),
            quote!(#stream_type::Item: Ord + #(#partialord_all)+* + Clone + Debug),
        ]
    });

    let where_clause = quote!(
        where #(#where_conditions),*
    );

    let stream_types = idents.iter().map(|ident| ident.stream_type.clone());

    let generics = quote!(
        <#(#stream_types),*>
    );

    let context = Context {
        name: &name,
        generics: &generics,
        where_clause: &where_clause,
        idents: &idents,
        mixed_ty: &format_ident!("Mixed{}", idents.len()),
    };

    let properties = idents.iter().flat_map(
        |Idents {
             stream_name,
             stream_type,
             last_name,
             pending_name,
             done_name,
             ..
         }| {
            [
                quote!(#[pin] #stream_name: Fuse<#stream_type>),
                quote!(#last_name: Option<#stream_type::Item>),
                quote!(#pending_name: Option<#stream_type::Item>),
                quote!(#done_name: bool),
            ]
        },
    );

    let impl_new = gen_impl_new(&context);
    let impl_stream = gen_impl_stream(&context);

    quote!(
        #[pin_project::pin_project]
        pub struct #name #generics #where_clause {
            #(#properties),*
        }

        #impl_new
        #impl_stream
    )
}

#[proc_macro]
pub fn sync_stream_struct_proc(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let structs = (2..=12).map(gen_sync_struct);

    quote!(
        use std::{
            fmt::Debug,
            pin::Pin,
        };
        use futures::{
            stream::Fuse,
            task::{
                Context,
                Poll,
            },
            Stream,
            StreamExt,
        };

        #(#structs)*
    )
    .into()
}
