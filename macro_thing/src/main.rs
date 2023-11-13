use macro_thing_core::{Inquire, InquireOption};
use macro_thing_derive::Inquire;

fn main() {
	enum AddressKnowledge {
		Registration(String),
		LastKnown(String),
	}

	// #[derive(Inquire)]
	struct Name {
		pub first: String,
		pub middle: InquireOption<String>,
		pub last: String,
	}

	#[derive(Inquire)]
	enum Zaimodavec {
		#[display("Physical person")]
		Physical {
			// #[question("Name of zaimodavec")]
			// name: Name,
			// #[question("Address of zaimodavec")]
			// address: AddressKnowledge,
			// #[question("KLADR code")]
			// kladr: Option<String>,
			#[question("Birth place of zaimodavec")]
			birth_place: String,
		},
		#[display("Individual entrepreneur")]
		IndividualEntrepreneur {
			// name: Name,
			// address: Option<AddressKnowledge>,
			kladr: String,
			birth_place: String,
		},
		LegalEntity {
			inn: String,
			name: String,
			address: String,
		},
	}

	let zd = Zaimodavec::inquire("123");
	// println!("Resulting enum: {:#?}", zd);
}

// #[cfg(test)]
// mod tests {
// 	use macro_thing_core::InquireDerive;
// 	use macro_thing_derive::InquireDerive;

// 	#[test]
// 	fn it_works() {
// 		#[derive(Debug)]
// 		enum AddressKnowledge {
// 			Registration(String),
// 			LastKnown(String),
// 		}

// 		struct Name {
// 			pub first: String,
// 			pub middle: Option<String>,
// 			pub last: String,
// 		}

// 		#[derive(InquireDerive)]
// 		enum Zaimodavec {
// 			#[display("Physical person")]
// 			Physical {
// 				// #[question("Name of zaimodavec")]
// 				// name: Name,
// 				// #[question("Address of zaimodavec")]
// 				// address: AddressKnowledge,
// 				// #[question("KLADR code")]
// 				// kladr: Option<String>,
// 				#[question("Birth place of zaimodavec")]
// 				birth_place: String,
// 			},
// 			#[display("Individual entrepreneur")]
// 			IndividualEntrepreneur {
// 				// name: Name,
// 				// address: Option<AddressKnowledge>,
// 				kladr: String,
// 				birth_place: String,
// 			},
// 			LegalEntity {
// 				inn: String,
// 				name: String,
// 				address: String,
// 			},
// 		}
// 	}
// }
