#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use macro_thing::Inquire;
fn main() {
    enum Test {
        A(String),
    }
    impl Inquire for Test {
        fn inquire(question: &str, mut __depth_counter: usize) -> Option<Self> {
            use Test::*;
            let chosen_variant = inquire::Select::new(question, ["A"].to_vec())
                .prompt()
                .ok()?;
            match chosen_variant {
                "A" => {
                    Some(Test::A {
                        0: {
                            __depth_counter += 1;
                            let ans = ::macro_thing::Question::<_>::new("(1 of 1) value")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
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
}
