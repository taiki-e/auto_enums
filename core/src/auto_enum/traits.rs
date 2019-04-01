use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::*;

use super::Arg;

pub(super) fn collect_impl_traits(args: &mut Stack<Arg>, ty: &mut Type) {
    if let Some(traits) = collect(ty) {
        parse(args, traits);
    }
}

fn collect(ty: &mut Type) -> Option<Stack<Path>> {
    let mut traits = Stack::new();
    ImplTraits::new(&mut traits).visit_type_mut(ty);

    if traits.is_empty() {
        None
    } else {
        Some(traits)
    }
}

fn parse(args: &mut Stack<Arg>, traits: Stack<Path>) {
    traits.into_iter().map(Arg::from).for_each(|t| {
        if !args.contains(&t) && TRAITS.contains(&&*t.to_trimed_string()) {
            args.push(t);
        }
    });
}

struct ImplTraits<'a> {
    traits: &'a mut Stack<Path>,
}

impl<'a> ImplTraits<'a> {
    fn new(traits: &'a mut Stack<Path>) -> Self {
        Self { traits }
    }
}

impl VisitMut for ImplTraits<'_> {
    fn visit_type_impl_trait_mut(&mut self, ty: &mut TypeImplTrait) {
        visit_mut::visit_type_impl_trait_mut(self, ty);

        ty.bounds.iter().for_each(|ty| {
            if let TypeParamBound::Trait(ty) = ty {
                self.traits.push(path(ty.path.segments.iter().map(|ty| ty.ident.clone().into())));
            }
        });
    }
}

const TRAITS: &[&str] = &[
    "Clone",
    "Copy",
    "PartialEq",
    "Eq",
    "PartialOrd",
    "Ord",
    "Hash",
    // core
    "AsRef",
    "AsMut",
    "Debug",
    "fmt::Debug",
    "Display",
    "fmt::Display",
    "fmt::Binary",
    "fmt::LowerExp",
    "fmt::LowerHex",
    "fmt::Octal",
    "fmt::Pointer",
    "fmt::UpperExp",
    "fmt::UpperHex",
    "fmt::Write",
    "Iterator",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "FusedIterator",
    "TrustedLen",
    "Extend",
    "Deref",
    "DerefMut",
    "Index",
    "IndexMut",
    "RangeBounds",
    "Fn",
    "FnMut",
    "FnOnce",
    "Generator",
    "Future",
    // std
    "Read",
    "io::Read",
    "BufRead",
    "io::BufRead",
    "Write",
    "io::Write",
    "Seek",
    "io::Seek",
    "Error",
    "error::Error",
];
