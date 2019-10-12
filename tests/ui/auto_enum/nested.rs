use auto_enums::auto_enum;

#[auto_enum(Iterator)]
fn match_nop(x: usize) -> impl Iterator<Item = i32> {
    match x {
        0 => 1..8,
        3 => {
            // This strange formatting is a rustfmt bug.
#[nested] //~ ERROR E0308
            2..=10
        }
        _ => (0..2).map(|x| x + 1),
    }
}

#[auto_enum(Iterator)]
fn if_nop(x: usize) -> impl Iterator<Item = i32> {
    if x == 0 {
        1..8
    } else if x > 3 {
        // This strange formatting is a rustfmt bug.
#[nested] //~ ERROR E0308
        2..=10
    } else {
        (0..2).map(|x| x + 1)
    }
}

fn main() {}
