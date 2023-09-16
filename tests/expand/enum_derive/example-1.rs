// SPDX-License-Identifier: Apache-2.0 OR MIT

fn foo(x: i32) -> impl Iterator<Item = i32> {
    #[::auto_enums::enum_derive(Iterator)]
    enum __Enum1<__T1, __T2> {
        __T1(__T1),
        __T2(__T2),
    }

    match x {
        0 => __Enum1::__T1(1..10),
        _ => __Enum1::__T2(vec![5, 10].into_iter()),
    }
}
