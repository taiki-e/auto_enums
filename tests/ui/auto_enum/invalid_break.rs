fn closure() -> impl Fn() {
    || break
}

fn async_block() -> impl std::future::Future<Output = ()> {
    async { break }
}

fn main() {}
