// SPDX-License-Identifier: Apache-2.0 OR MIT

fn closure() -> impl Fn() {
    || break
}

fn async_block() -> impl std::future::Future<Output = ()> {
    async { break }
}

fn main() {}
