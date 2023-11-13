use std::{fmt::Debug, marker::PhantomData, rc::Rc, str::FromStr};

use inquire::{
	required,
	validator::{Validation, ValueRequiredValidator},
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
		Self {
			message,
			_marker: PhantomData,
		}
	}

	pub fn ask(self) -> Option<T> {
		T::inquire(self.message)
		// let response = inquire::CustomType::<RcStr<T>>::new(self.message)
		// 	.with_error_message("Invalid input")
		// 	.with_validator(|x: &RcStr<T>| {
		// 		if x.to_string().is_empty() {
		// 			Ok(Validation::Invalid("Input must not be empty".into()))
		// 		} else {
		// 			Ok(Validation::Valid)
		// 		}
		// 	})
		// 	.with_parser(&|x| x)
		// 	.prompt()
		// 	.ok()?;
		// Rc::into_inner(response.0)
	}
}

// pub struct EnumQuestion {
// 	variants: Vec<&'static str>,
// }

// impl EnumQuestion {
// 	pub fn new(variants: Vec<&'static str>) -> Self {
// 		Self { variants }
// 	}

// 	pub fn ask(self) -> Option<&'static str> {
// 		inquire::Select::new("Select variant", self.variants).prompt().ok()
// 	}
// }

pub struct InquireOption<T>(Option<T>);

impl<T> From<Option<T>> for InquireOption<T> {
	fn from(x: Option<T>) -> Self {
		Self(x)
	}
}

pub trait Inquire {
	fn inquire(message: &str) -> Option<Self>
	where
		Self: Sized;
}

impl<T> Inquire for T
where
	T: FromStr + ToString,
	T::Err: Debug,
{
	fn inquire(question: &str) -> Option<Self> {
		let response = inquire::CustomType::<RcStr<T>>::new(question)
			.with_error_message("Invalid input")
			.with_validator(|x: &RcStr<T>| {
				if x.to_string().is_empty() {
					Ok(Validation::Invalid("Input must not be empty".into()))
				} else {
					Ok(Validation::Valid)
				}
			})
			.prompt()
			.ok()?;
		Rc::into_inner(response.0)
	}
}

impl<T> Inquire for InquireOption<T>
where
	T: FromStr + ToString,
	T::Err: Debug,
{
	fn inquire(question: &str) -> Option<Self> {
		let response = inquire::CustomType::<RcStr<T>>::new(question)
			.with_error_message("Invalid input")
			.with_validator(|x: &RcStr<T>| {
				if x.to_string().is_empty() {
					Ok(Validation::Invalid("Input must not be empty".into()))
				} else {
					Ok(Validation::Valid)
				}
			})
			.prompt_skippable()
			.ok()?;
		response.map(|x| Rc::into_inner(x.0).into())
	}
}
