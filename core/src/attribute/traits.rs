use smallvec::SmallVec;
use syn::{
    visit::{self, Visit},
    *,
};

use crate::utils::*;

use super::*;

pub(super) fn collect_impl_traits(ty: &Type) -> Option<SmallVec<[Path; 4]>> {
    let mut traits = SmallVec::new();
    ImplTraits::new(&mut traits).visit_type(ty);

    if traits.is_empty() {
        None
    } else {
        Some(traits)
    }
}

struct ImplTraits<'a> {
    traits: &'a mut SmallVec<[Path; 4]>,
}

impl<'a> ImplTraits<'a> {
    fn new(traits: &'a mut SmallVec<[Path; 4]>) -> Self {
        Self { traits }
    }
}

impl<'a, 'ast> Visit<'ast> for ImplTraits<'a> {
    fn visit_type_impl_trait(&mut self, ty: &'ast TypeImplTrait) {
        visit::visit_type_impl_trait(self, ty);
        ty.bounds.iter().for_each(|ty| {
            if let TypeParamBound::Trait(ty) = ty {
                self.traits.push(path(
                    ty.path.segments.iter().map(|ty| ty.ident.clone().into()),
                ));
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
    "Deref",
    "DerefMut",
    "Index",
    "IndexMut",
    "Fn",
    "FnMut",
    "FnOnce",
    "RangeBounds",
    "AsRef",
    "AsMut",
    "Iterator",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "FusedIterator",
    "TrustedLen",
    "Extend",
    "Future",
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
];

pub(super) fn parse_impl_traits(args: &mut Stack<Arg>, traits: SmallVec<[Path; 4]>) {
    traits.into_iter().map(|t| t.into()).for_each(|t| {
        if !args.contains(&t) && TRAITS.contains(&&*t.to_trimed_string()) {
            args.push(t)
        }
    });
}
