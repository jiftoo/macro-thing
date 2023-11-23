#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use macro_thing_core::{Inquire, InquireOption, Question};
use macro_thing_derive::Inquire;
fn main() {
    enum AddressKnowledge {
        Registration { value: String },
        LastKnown { value: String },
    }
    impl Inquire for AddressKnowledge {
        fn inquire(question: &str, mut __depth_counter: usize) -> Option<Self> {
            use ::macro_thing_core::*;
            use AddressKnowledge::*;
            let chosen_variant = inquire::Select::new(
                    question,
                    ["Registration", "LastKnown"].to_vec(),
                )
                .prompt()
                .ok()?;
            match chosen_variant {
                "Registration" => {
                    Some(AddressKnowledge::Registration {
                        value: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(1 of 1) value:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                    })
                }
                "LastKnown" => {
                    Some(AddressKnowledge::LastKnown {
                        value: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(1 of 1) value:")
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
    #[automatically_derived]
    impl ::core::fmt::Debug for AddressKnowledge {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddressKnowledge::Registration { value: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Registration",
                        "value",
                        &__self_0,
                    )
                }
                AddressKnowledge::LastKnown { value: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "LastKnown",
                        "value",
                        &__self_0,
                    )
                }
            }
        }
    }
    struct Name {
        #[question("First name")]
        pub first: String,
        #[question("Middle name")]
        pub middle: InquireOption<String>,
        #[question("Last name")]
        pub last: String,
    }
    impl Inquire for Name {
        fn inquire(question: &str, mut __depth_counter: usize) -> Option<Self> {
            use ::macro_thing_core::*;
            Some(Name {
                first: {
                    __depth_counter += 1;
                    let ans = Question::<_>::new("(1 of 3) First name:")
                        .ask(__depth_counter)?;
                    __depth_counter -= 1;
                    ans
                },
                middle: {
                    __depth_counter += 1;
                    let ans = Question::<_>::new("(2 of 3) Middle name:")
                        .ask(__depth_counter)?;
                    __depth_counter -= 1;
                    ans
                },
                last: {
                    __depth_counter += 1;
                    let ans = Question::<_>::new("(3 of 3) Last name:")
                        .ask(__depth_counter)?;
                    __depth_counter -= 1;
                    ans
                },
            })
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Name",
                "first",
                &self.first,
                "middle",
                &self.middle,
                "last",
                &&self.last,
            )
        }
    }
    enum Zaimodavec {
        #[display("Physical person")]
        Physical {
            #[question("Name of zaimodavec")]
            name: Name,
            #[question("Address of zaimodavec")]
            address: AddressKnowledge,
            #[optional]
            #[question("KLADR code")]
            kladr: Option<String>,
            #[question("Birth place of zaimodavec")]
            birth_place: String,
        },
        #[display("Individual entrepreneur")]
        IndividualEntrepreneur { kladr: String, birth_place: String },
        LegalEntity { inn: String, name: String, address: String },
    }
    impl Inquire for Zaimodavec {
        fn inquire(question: &str, mut __depth_counter: usize) -> Option<Self> {
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
                "Physical person" => {
                    Some(Zaimodavec::Physical {
                        name: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(1 of 4) Name of zaimodavec:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                        address: {
                            __depth_counter += 1;
                            let ans = Question::<
                                _,
                            >::new("(2 of 4) Address of zaimodavec:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                        kladr: {
                            __depth_counter += 1;
                            let ans = Question::<
                                InquireOption<_>,
                            >::new("(3 of 4) [optional] KLADR code:")
                                .ask(__depth_counter)?
                                .into_option();
                            __depth_counter -= 1;
                            ans
                        },
                        birth_place: {
                            __depth_counter += 1;
                            let ans = Question::<
                                _,
                            >::new("(4 of 4) Birth place of zaimodavec:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                    })
                }
                "Individual entrepreneur" => {
                    Some(Zaimodavec::IndividualEntrepreneur {
                        kladr: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(1 of 2) kladr:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                        birth_place: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(2 of 2) birth_place:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                    })
                }
                "LegalEntity" => {
                    Some(Zaimodavec::LegalEntity {
                        inn: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(1 of 3) inn:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                        name: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(2 of 3) name:")
                                .ask(__depth_counter)?;
                            __depth_counter -= 1;
                            ans
                        },
                        address: {
                            __depth_counter += 1;
                            let ans = Question::<_>::new("(3 of 3) address:")
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Zaimodavec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Zaimodavec::Physical {
                    name: __self_0,
                    address: __self_1,
                    kladr: __self_2,
                    birth_place: __self_3,
                } => {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Physical",
                        "name",
                        __self_0,
                        "address",
                        __self_1,
                        "kladr",
                        __self_2,
                        "birth_place",
                        &__self_3,
                    )
                }
                Zaimodavec::IndividualEntrepreneur {
                    kladr: __self_0,
                    birth_place: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "IndividualEntrepreneur",
                        "kladr",
                        __self_0,
                        "birth_place",
                        &__self_1,
                    )
                }
                Zaimodavec::LegalEntity {
                    inn: __self_0,
                    name: __self_1,
                    address: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "LegalEntity",
                        "inn",
                        __self_0,
                        "name",
                        __self_1,
                        "address",
                        &__self_2,
                    )
                }
            }
        }
    }
    let zd = Zaimodavec::inquire("Select zaimodavec", 0);
    {
        ::std::io::_print(format_args!("Resulting enum: {0:#?}\n", zd));
    };
}
