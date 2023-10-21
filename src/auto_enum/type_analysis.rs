// SPDX-License-Identifier: Apache-2.0 OR MIT

use quote::ToTokens;
use syn::{
    visit_mut::{self, VisitMut},
    Path, Type, TypeImplTrait, TypeParamBound,
};

use crate::utils::path;

pub(super) fn collect_impl_trait(args: &[Path], traits: &mut Vec<Path>, ty: &mut Type) -> bool {
    struct CollectImplTrait<'a> {
        args: &'a [Path],
        traits: &'a mut Vec<Path>,
        has_impl_trait: bool,
    }

    impl VisitMut for CollectImplTrait<'_> {
        fn visit_type_impl_trait_mut(&mut self, node: &mut TypeImplTrait) {
            visit_mut::visit_type_impl_trait_mut(self, node);

            for ty in &node.bounds {
                if let TypeParamBound::Trait(ty) = ty {
                    let ty = path(ty.path.segments.iter().map(|ty| ty.ident.clone().into()));
                    let ty_str = ty.to_token_stream().to_string();
                    let ty_trimmed = ty_str.replace(' ', "");
                    if TRAITS.contains(&&*ty_trimmed)
                        && !self.args.iter().any(|x| x.to_token_stream().to_string() == ty_str)
                    {
                        self.has_impl_trait = true;
                        self.traits.push(ty);
                    }
                }
            }
        }
    }

    let mut visitor = CollectImplTrait { args, traits, has_impl_trait: false };
    visitor.visit_type_mut(ty);
    visitor.has_impl_trait
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
    "Coroutine",
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
