// SPDX-License-Identifier: Apache-2.0 OR MIT

use quote::ToTokens as _;
use syn::{
    Path, Type, TypeImplTrait, TypeParamBound,
    visit_mut::{self, VisitMut},
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
                if let TypeParamBound::Trait(orig_ty) = ty {
                    let ty = path(orig_ty.path.segments.iter().map(|ty| ty.ident.clone().into()));
                    let ty_str = ty.to_token_stream().to_string();
                    let orig_ty_str = orig_ty.to_token_stream().to_string();
                    if TRAITS.contains(&&*ty_str)
                        && !self.args.iter().any(|x| {
                            let arg_str = x.to_token_stream().to_string();
                            // Some derives (ie: Into) need to check against the derive argument itself
                            arg_str == ty_str || arg_str == orig_ty_str
                        })
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
    "Into",
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
