use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn def_site(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..8,
        3 => {
            #[never]
            return __Enumdef_site::__Variant0(2..4);
        }
        _ => (0..2).map(|x| x + 1),
    }
}

fn main() {}
