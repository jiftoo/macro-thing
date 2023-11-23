// use macro_thing_core::{Inquire, Question, InquireOrDefault};
// use macro_thing_derive::Inquire;

// fn main() {
// 	#[derive(Inquire, Debug)]
// 	enum AddressKnowledge {
// 		Registration{
// 			value: String
// 		},
// 		LastKnown {
// 			value: String
// 		},
// 	}

// 	#[derive(Inquire, Debug)]
// 	struct Name {
// 		#[question("First name")]
// 		pub first: String,
// 		#[question("Middle name")]
// 		pub middle: Option<String>,
// 		#[question("Last name")]
// 		pub last: String,
// 		#[question("Address for some reason")]
// 		pub address: AddressKnowledge,
// 	}

// 	#[derive(Inquire, Debug)]
// 	enum Zaimodavec {
// 		#[display("Physical person")]
// 		Physical {
// 			#[question("Name of zaimodavec")]
// 			name: Name,
// 			#[question("Address of zaimodavec")]
// 			address: AddressKnowledge,
// 			#[optional]
// 			#[question("KLADR code")]
// 			kladr: String,
// 			#[question("Birth place of zaimodavec")]
// 			birth_place: String,
// 		},
// 		#[display("Individual entrepreneur")]
// 		IndividualEntrepreneur {
// 			// name: Name,
// 			// address: Option<AddressKnowledge>,
// 			kladr: String,
// 			birth_place: String,
// 		},
// 		LegalEntity {
// 			inn: String,
// 			name: String,
// 			address: String,
// 		},
// 	}

// 	let zd = Zaimodavec::inquire("Select zaimodavec", 0);
// 	println!("Resulting enum: {:#?}", zd);
// }

// // #[cfg(test)]
// // mod tests {
// // 	use macro_thing_core::InquireDerive;
// // 	use macro_thing_derive::InquireDerive;

// // 	#[test]
// // 	fn it_works() {
// // 		#[derive(Debug)]
// // 		enum AddressKnowledge {
// // 			Registration(String),
// // 			LastKnown(String),
// // 		}

// // 		struct Name {
// // 			pub first: String,
// // 			pub middle: Option<String>,
// // 			pub last: String,
// // 		}

// // 		#[derive(InquireDerive)]
// // 		enum Zaimodavec {
// // 			#[display("Physical person")]
// // 			Physical {
// // 				// #[question("Name of zaimodavec")]
// // 				// name: Name,
// // 				// #[question("Address of zaimodavec")]
// // 				// address: AddressKnowledge,
// // 				// #[question("KLADR code")]
// // 				// kladr: Option<String>,
// // 				#[question("Birth place of zaimodavec")]
// // 				birth_place: String,
// // 			},
// // 			#[display("Individual entrepreneur")]
// // 			IndividualEntrepreneur {
// // 				// name: Name,
// // 				// address: Option<AddressKnowledge>,
// // 				kladr: String,
// // 				birth_place: String,
// // 			},
// // 			LegalEntity {
// // 				inn: String,
// // 				name: String,
// // 				address: String,
// // 			},
// // 		}
// // 	}
// // }

// use macro_thing::Inquire;

fn main() {
	// #[derive(Inquire)]
	// enum Test {
	// 	A(String),
	// }
}
