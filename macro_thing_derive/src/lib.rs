#![allow(clippy::all)]
#![allow(unused_imports)]

use std::collections::HashMap;

use macro_thing_core::Question;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{format_ident, quote};
use syn::{
	parse, parse_macro_input, parse_quote, Attribute, DeriveInput, ItemEnum, ItemStruct,
	LitStr, Path, Token, Type,
};

#[derive(Debug, Clone)]
struct FieldConfig {
	question: String,
	optional: bool,
}

impl FieldConfig {
	fn format_question(&self, question_idx: usize, question_max: usize) -> String {
		format!(
			"({} of {}) {}{}:",
			question_idx + 1,
			question_max,
			{
				if self.optional {
					"[optional] "
				} else {
					""
				}
			},
			self.question
		)
	}
}

#[proc_macro_derive(Inquire, attributes(question, display, optional))]
pub fn inquire_macro(input: TokenStream) -> TokenStream {
	match syn::parse::<ItemEnum>(input.clone()) {
		Ok(item_enum) => expand_enum(item_enum),
		Err(_) => {
			let struct_input = parse_macro_input!(input as ItemStruct);
			expand_struct_from_item_struct(struct_input)
		}
	}
}

const DEPTH_COUNTER_IDENT: &str = "__depth_counter";

fn expand_struct_from_item_struct(item_struct: ItemStruct) -> TokenStream {
	let ItemStruct { ident, fields, .. } = item_struct;
	let questions: Vec<_> = fields
		.iter()
		.map(|field| {
			let field_ident =
				field.ident.as_ref().cloned().expect("Tuple structs not supported");

			// fields allow an #[optional] attribute
			let optional =
				field.attrs.iter().find(|x| x.path().is_ident("optional")).is_some();

			// and a #[question] attribute
			if let Some(question) =
				field.attrs.iter().find(|x| x.path().is_ident("question"))
			{
				let custom_question = question.parse_args::<LitStr>().unwrap().value();
				(field_ident, FieldConfig { question: custom_question, optional })
			} else {
				let default_question = field_ident.to_string();
				(field_ident, FieldConfig { question: default_question, optional })
			}
		})
		.collect();

	let struct_tokens = expand_struct(parse_quote!(#ident), questions);
	let depth_counter_ident = format_ident!("{DEPTH_COUNTER_IDENT}");

	quote! {
		impl Inquire for #ident {
			fn inquire(question: &str, mut #depth_counter_ident: usize) -> Option<Self> {
				use ::macro_thing_core::*;
				#struct_tokens
			}
		}
	}
	.into()
}

fn expand_enum(ItemEnum { ident: main_ident, variants, .. }: ItemEnum) -> TokenStream {
	// Map of variants renamed with #[display] attribute
	let mut renamed_variants = HashMap::new();

	// Populate the map
	for variant in &variants {
		// variants allow a #[display] attribute
		if let Some(display) = variant.attrs.iter().find(|x| x.path().is_ident("display"))
		{
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
	let mut questions: HashMap<&syn::Variant, Vec<(Ident, FieldConfig)>> = HashMap::new();

	for variant in &variants {
		for field in variant.fields.iter() {
			let questions_for_variant = questions.entry(variant).or_insert(Vec::new());

			let field_ident =
				field.ident.as_ref().cloned().expect("Tuple structs not supported");

			// fields allow an #[optional] attribute
			let optional =
				field.attrs.iter().find(|x| x.path().is_ident("optional")).is_some();

			// and a #[question] attribute
			if let Some(question) =
				field.attrs.iter().find(|x| x.path().is_ident("question"))
			{
				let custom_question = question.parse_args::<LitStr>().unwrap().value();
				questions_for_variant.push((
					field_ident,
					FieldConfig { question: custom_question, optional },
				));
			} else {
				let default_question = field_ident.to_string();
				questions_for_variant.push((
					field_ident,
					FieldConfig { question: default_question, optional },
				));
			}
		}
	}
	let match_arms = questions
		.into_iter()
		.map(|(original_variant, questions)| {
			// respect #[display] attribute
			let renamed_variant = renamed_variants
				.get(&original_variant.ident)
				.cloned()
				.unwrap_or_else(|| original_variant.ident.to_string());

			let original_variant_ident = &original_variant.ident;
			let struct_tokens = expand_struct(
				parse_quote!(#main_ident::#original_variant_ident),
				questions,
			);

			quote! {
				#renamed_variant => {
					#struct_tokens
				}
			}
		})
		.collect::<Vec<_>>();

		let depth_counter_ident = format_ident!("{DEPTH_COUNTER_IDENT}");

	quote! {
		impl Inquire for #main_ident {
			fn inquire(question: &str, mut #depth_counter_ident: usize) -> Option<Self> {
				use ::macro_thing_core::*;
				use #main_ident::*;
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

fn expand_struct(
	struct_name: Path,
	questions: Vec<(Ident, FieldConfig)>,
) -> proc_macro2::TokenStream {
	let depth_counter_ident = format_ident!("{DEPTH_COUNTER_IDENT}");

	let total_questions = questions.len();
	let statements: Vec<_> = questions
		.into_iter()
		.enumerate()
		.map(|(i, (field_ident, ref cfg @ FieldConfig { ref question, optional }))| {
			let question = cfg.format_question(i, total_questions);
			let type_param: Type =
				if optional { parse_quote!(InquireOption<_>) } else { parse_quote!(_) };
			let into_option_call = optional.then(|| quote! {.into_option()});
			let question_statement = quote! {
				#field_ident: {
					#depth_counter_ident += 1;
					let ans = Question::<#type_param>::new(#question).ask(#depth_counter_ident)?#into_option_call;
					#depth_counter_ident -= 1;
					ans
				}
			};
			question_statement
		})
		.collect();

	quote! {
		Some(#struct_name {
			#(
				#statements,
			)*
		})
	}
}
