use auto_enums::auto_enum;

#[auto_enum]
fn b(x: bool) -> Option<impl Iterator<Item = u8>> {
    Some(
        #[auto_enum(Iterator)]
        if x { std::iter::once(0) } else { std::iter::repeat(1) },
    )
}

#[auto_enum]
fn c(x: bool) -> Option<impl Iterator<Item = u8>> {
    Some(
        #[auto_enum(Iterator)]
        match x {
            true => std::iter::once(0),
            _ => std::iter::repeat(1),
        },
    )
}

fn main() {}
