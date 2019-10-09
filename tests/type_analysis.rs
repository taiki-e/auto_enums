#![cfg(feature = "type_analysis")]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use auto_enums::auto_enum;

#[test]
fn type_analysis() {
    #[auto_enum] // there is no need to specify std library's traits
    fn foo(x: i32) -> impl Iterator<Item = i32> {
        match x {
            0 => 1..10,
            _ => vec![5, 10].into_iter(),
        }
    }
}
