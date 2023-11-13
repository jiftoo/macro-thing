#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use macro_thing_core::{Inquire, InquireOption};
use macro_thing_derive::Inquire;
fn main() {
    enum AddressKnowledge {
        Registration(String),
        LastKnown(String),
    }
    struct Name {
        pub first: String,
        pub middle: InquireOption<String>,
        pub last: String,
    }
    enum Zaimodavec {
        #[display("Physical person")]
        Physical { #[question("Birth place of zaimodavec")] birth_place: String },
        #[display("Individual entrepreneur")]
        IndividualEntrepreneur { kladr: String, birth_place: String },
        LegalEntity { inn: String, name: String, address: String },
    }
    impl Inquire for Zaimodavec {
        fn inquire(question: &str) -> Option<Self> {
            use ::macro_thing_core::*;
            use Zaimodavec::*;
            let chosen_variant = inquire::Select::new(
                    question,
                    ["Physical person", "Individual entrepreneur", "LegalEntity"]
                        .to_vec(),
                )
                .prompt()
                .ok()?;
            match chosen_variant {
                "LegalEntity" => {
                    Some(Zaimodavec::LegalEntity {
                        inn: Question::new("(1 of 3) inn:").ask()?,
                        name: Question::new("(2 of 3) name:").ask()?,
                        address: Question::new("(3 of 3) address:").ask()?,
                    })
                }
                "Physical person" => {
                    Some(Zaimodavec::Physical {
                        birth_place: Question::new("(1 of 1) birth_place:").ask()?,
                    })
                }
                "Individual entrepreneur" => {
                    Some(Zaimodavec::IndividualEntrepreneur {
                        kladr: Question::new("(1 of 2) kladr:").ask()?,
                        birth_place: Question::new("(2 of 2) birth_place:").ask()?,
                    })
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("Inquire bug"),
                        ),
                    );
                }
            }
        }
    }
    let zd = Zaimodavec::inquire("123");
}
