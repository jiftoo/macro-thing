use std::{borrow::Cow, fmt::Debug, marker::PhantomData, rc::Rc, str::FromStr};

use inquire::{
	error::InquireResult,
	required,
	validator::{Validation, ValueRequiredValidator}, CustomUserError,
};

pub use inquire;

#[derive(Debug)]
pub struct RcStr<T>(Rc<T>);

impl<T> Clone for RcStr<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T: FromStr> FromStr for RcStr<T> {
	type Err = T::Err;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(Rc::new(T::from_str(s)?)))
	}
}

impl<T: ToString> ToString for RcStr<T> {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}
pub struct Question<T: Inquire> {
	message: &'static str,
	_marker: PhantomData<T>,
}
impl<T: Inquire> Question<T> {
	pub fn new(message: &'static str) -> Self {
		Self { message, _marker: PhantomData }
	}

	pub fn ask(self, depth: usize) -> Option<T> {
		T::inquire(self.message, depth)
	}
}

pub trait Inquire {
	fn inquire(message: &str, depth: usize) -> Option<Self>
	where
		Self: Sized;
}

pub enum InqureKind {
	Custom,
	Text,
}

pub struct InquireHelper<T> {
	question: String,
	depth: usize,
	error: String,
	validator: Box<dyn inquire::validator::CustomTypeValidator<T>>,
	optional: bool,
}

impl<T: Clone + FromStr + ToString + 'static> InquireHelper<T> {
	pub fn new(question: &str, depth: usize) -> Self {
		Self {
			question: question.into(),
			depth,
			error: "Invalid input".into(),
			validator: Box::new(|x: &T| {
				if x.to_string().is_empty() {
					Ok(Validation::Invalid("Input must not be empty".into()))
				} else {
					Ok(Validation::Valid)
				}
			}),
			optional: false,
		}
	}

	pub fn optional(mut self) -> Self {
		self.optional = true;
		self.validator = Box::new(|x: &T| Ok(Validation::Valid));
		self
	}

	pub fn with_error_message(mut self, error: &str) -> Self {
		self.error = error.into();
		self
	}

	pub fn with_validator<F: 'static + Clone>(mut self, validator: F) -> Self
	where
		F: Fn(&T) -> Result<Validation, CustomUserError>,
	{
		self.validator = Box::new(validator);
		self
	}

	pub fn prompt(self) -> InquireResult<T> {
		let message = if self.optional {
			Self::format_question_optional(&self.question, self.depth)
		} else {
			Self::format_question(&self.question, self.depth)
		};

		inquire::CustomType::<T>::new(&message)
			.with_error_message(&self.error)
			.with_validator(move |x: &_| self.validator.validate(x))
			.prompt()
	}

	fn format_question(question: &str, depth: usize) -> String {
		format!("{}{}:", "-".repeat(depth - 1), question)
	}

	fn format_question_optional(question: &str, depth: usize) -> String {
		format!("{} [optional] {}:", "-".repeat(depth - 1), question)
	}
}

impl Inquire for String {
	fn inquire(question: &str, depth: usize) -> Option<Self> {
		let response = inquire::CustomType::<String>::new(&format!(
			"{}{}:",
			"-".repeat(depth - 1),
			question
		))
		.with_error_message("Invalid input")
		.with_validator(|x: &String| {
			if x.to_string().is_empty() {
				Ok(Validation::Invalid("Input must not be empty".into()))
			} else {
				Ok(Validation::Valid)
			}
		})
		.prompt()
		.ok()?;
		Some(response)
	}
}

impl<T: ToString + FromStr> Inquire for Option<T> {
	fn inquire(question: &str, depth: usize) -> Option<Self> {
		let response = inquire::CustomType::<RcStr<T>>::new(&format!(
			"{} [optional] {}:",
			"-".repeat(depth - 1),
			question
		))
		.with_error_message("Invalid input")
		.prompt_skippable()
		.ok()?
		// just hitting enter should also return None
		.and_then(|x| if x.to_string().is_empty() { None } else { Some(x) });

		Some(response.map(|x| Rc::into_inner(x.0).unwrap()))
	}
}

pub struct InquireOrDefault<T: ToString + FromStr>(T);

impl<T: ToString + FromStr + Default> Inquire for InquireOrDefault<T> {
	fn inquire(question: &str, depth: usize) -> Option<Self> {
		match Option::<T>::inquire(question, depth) {
			Some(x) => Some(Self(x.unwrap())),
			None => Some(Self(T::default())),
		}
	}
}

impl<T: ToString + FromStr> InquireOrDefault<T> {
	pub fn into_inner(self) -> T {
		self.0
	}
}
