#![allow(clippy::all)]
#![allow(unused_imports)]

use std::collections::HashMap;

use macro_thing_core::Question;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{format_ident, quote};
use syn::{
	parse, parse_macro_input, parse_quote, Attribute, DeriveInput, ItemEnum, ItemStruct, LitStr,
	Path, Token,
};

#[proc_macro_derive(Inquire, attributes(question, display))]
pub fn inquire_macro(input: TokenStream) -> TokenStream {
	let ItemEnum {
		ident: main_ident,
		variants,
		..
	} = parse_macro_input!(input as ItemEnum);

	// Map of variants renamed with #[display] attribute
	let mut renamed_variants = HashMap::new();

	// Populate the map
	for variant in &variants {
		// variants allow a #[display] attribute
		if let Some(display) = variant.attrs.iter().find(|x| x.path().is_ident("display")) {
			let display_as = display.parse_args::<LitStr>().unwrap().value();
			renamed_variants.insert(&variant.ident, display_as);
		}
	}

	// Now collect all variants, keeping the renamed ones
	// this is used at top level to build the first question
	// and the `match` statement
	let variant_idents = variants
		.iter()
		.map(|x| &x.ident)
		.map(|x| renamed_variants.get(x).cloned().unwrap_or_else(|| x.to_string()))
		.collect::<Vec<_>>();

	// map of a variant name to a list of questions for its fields
	let mut questions: HashMap<&syn::Variant, Vec<(Ident, String)>> = HashMap::new();

	for variant in &variants {
		let field_count = variant.fields.len();
		for (field_idx, field) in variant.fields.iter().enumerate() {
			let fmt_question = |x: &str| format!("({} of {}) {}:", field_idx + 1, field_count, x);

			let questions_for_variant = questions.entry(variant).or_insert(Vec::new());

			let field_ident = field.ident.as_ref().cloned().expect("Tuple structs not supported");

			// fields allow a #[question] attribute
			if let Some(question) = field.attrs.iter().find(|x| x.path().is_ident("display")) {
				let custom_question = question.parse_args::<LitStr>().unwrap().value();
				questions_for_variant.push((field_ident, custom_question));
			} else {
				let default_question = fmt_question(&field_ident.to_string());
				questions_for_variant.push((field_ident, default_question));
			}
		}
	}
	let match_arms = questions
		.iter()
		.map(|(original_variant, questions)| {
			let renamed_variant = renamed_variants
				.get(&original_variant.ident)
				.cloned()
				.unwrap_or_else(|| original_variant.ident.to_string());

			let original_variant_ident = &original_variant.ident;
			let struct_tokens = expand_struct(
				parse_quote!(#main_ident::#original_variant_ident),
				questions.clone(),
			);

			quote! {
				#renamed_variant => {
					#struct_tokens
				}
			}
		})
		.collect::<Vec<_>>();

	quote! {
		impl Inquire for #main_ident {
			fn inquire(question: &str) -> Option<Self> {
				use ::macro_thing_core::*;
				use #main_ident::*;
				// let chosen_variant = EnumQuestion::new([#(#variant_idents),*].to_vec()).ask()?;
				let chosen_variant = inquire::Select::new(question, [#(#variant_idents),*].to_vec()).prompt().ok()?;
				match chosen_variant {
					#(#match_arms)*
					_ => unreachable!("Inquire bug"),
				}
			}
		}
	}
	.into()
}

fn expand_struct(struct_name: Path, questions: Vec<(Ident, String)>) -> proc_macro2::TokenStream {
	// respect #[display] attribute
	// let renamed_variant = renamed_variants
	// 	.get(&struct_name.ident)
	// 	.cloned()
	// 	.unwrap_or_else(|| struct_name.ident.to_string());

	let statements: Vec<_> = questions
		.into_iter()
		.map(|(field_ident, question)| {
			let question_statement = quote! {
				#field_ident: Question::new(#question).ask()?
			};
			question_statement
		})
		.collect();

	quote! {
		// #renamed_variant => {
			Some(#struct_name {
				#(
					#statements,
				)*
			})
		// }
	}
}
