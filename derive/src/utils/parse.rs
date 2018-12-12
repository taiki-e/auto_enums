use std::{borrow::Cow, mem};

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    *,
};

use crate::utils::{Result, *};

pub(crate) type Data = EnumData;

struct Attributes(Vec<Attribute>);

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.call(Attribute::parse_outer).map(Attributes)
    }
}

pub(crate) struct EnumData {
    ident: Ident,
    generics: Generics,
    variants: Stack<Ident>,
    fields: Stack<Type>,
}

impl EnumData {
    pub(crate) fn impl_with_capacity<'a>(
        &'a self,
        capacity: usize,
        root: TokenStream,
    ) -> Result<EnumImpl<'a>> {
        EnumImpl::new(self, root, Vec::with_capacity(capacity))
    }

    pub(crate) fn impl_trait_with_capacity<'a>(
        &'a self,
        capacity: usize,
        root: TokenStream,
        trait_path: Path,
        supertraits_bounds: SmallVec<[Ident; 1]>,
        item: ItemTrait,
    ) -> Result<EnumImpl<'a>> {
        EnumImpl::from_trait(
            self,
            root,
            trait_path,
            Vec::with_capacity(capacity),
            item,
            supertraits_bounds,
        )
    }

    #[cfg(feature = "transpose_methods")]
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }
    #[cfg(feature = "transpose_methods")]
    pub(crate) fn generics(&self) -> &Generics {
        &self.generics
    }
    #[cfg(feature = "transpose_methods")]
    pub(crate) fn variants(&self) -> &[Ident] {
        &self.variants
    }
    pub(crate) fn fields(&self) -> &[Type] {
        &self.fields
    }
}

pub(crate) struct Trait {
    /// `AsRef`
    path: Path,
    /// `AsRef<T>`
    ty: Path,
    /// `!`
    bang: bool,
}

impl Trait {
    pub(crate) fn new(path: Path, ty: Path) -> Self {
        Trait {
            path,
            ty,
            bang: false,
        }
    }
}

pub(crate) struct EnumImpl<'a> {
    data: &'a EnumData,
    defaultness: bool,
    unsafety: bool,
    generics: Generics,
    trait_: Option<Trait>,
    self_ty: Box<Type>,
    items: Vec<ImplItem>,
    root: TokenStream,
    unsafe_code: bool,
}

pub(crate) fn build(impls: EnumImpl<'_>) -> TokenStream {
    impls.build()
}

impl<'a> EnumImpl<'a> {
    fn new(data: &'a EnumData, root: TokenStream, items: Vec<ImplItem>) -> Result<Self> {
        let ident = &data.ident;
        let (_, ty_generics, _) = data.generics.split_for_impl();
        syn::parse2(quote!(#ident #ty_generics))
            .map(|self_ty| EnumImpl {
                data,
                defaultness: false,
                unsafety: false,
                generics: data.generics.clone(),
                trait_: None,
                self_ty: Box::new(self_ty),
                items,
                root,
                unsafe_code: false,
            })
            .map_err(|e| e.to_string().into())
    }

    pub(crate) fn trait_(&mut self) -> &mut Option<Trait> {
        &mut self.trait_
    }
    #[cfg(feature = "transpose_methods")]
    pub(crate) fn self_ty(&mut self) -> &mut Type {
        &mut *self.self_ty
    }

    pub(crate) fn push_generic_param(&mut self, param: GenericParam) {
        self.generics.params.push(param);
    }

    pub(crate) fn push_where_predicate(&mut self, predicate: WherePredicate) {
        self.generics.make_where_clause().predicates.push(predicate);
    }

    pub(crate) fn push_item(&mut self, item: ImplItem) {
        self.items.push(item);
    }

    pub(crate) fn push_method(&mut self, item: TraitItemMethod) -> Result<()> {
        self._push_method(item, None)
    }

    #[allow(dead_code)]
    pub(crate) fn push_method_pin_ref(&mut self, item: TraitItemMethod) -> Result<()> {
        self._push_method(item, Some(SelfTypes::Pin(SelfPin::Ref)))
    }

    pub(crate) fn push_method_pin_mut(&mut self, item: TraitItemMethod) -> Result<()> {
        self._push_method(item, Some(SelfTypes::Pin(SelfPin::Mut)))
    }

    fn arms<F: FnMut(&Ident) -> TokenStream>(&self, f: F) -> TokenStream {
        let arms = self.data.variants.iter().map(f);
        quote!(#(#arms,)*)
    }

    fn trait_path(&self) -> Option<&Path> {
        self.trait_.as_ref().map(|t| &t.path)
    }

    fn _push_method(&mut self, item: TraitItemMethod, self_ty: Option<SelfTypes>) -> Result<()> {
        let method = {
            let mut args = item.sig.decl.inputs.iter();
            match args.next() {
                Some(FnArg::SelfRef(_)) | Some(FnArg::SelfValue(_)) if self_ty.is_none() => {}
                Some(FnArg::Captured(_arg)) if self_ty.is_some() => {}
                _ => Err("unsupported arg type")?,
            }
            let args: &Stack<_> = &args
                .map(|arg| match arg {
                    FnArg::Captured(arg) => Ok(&arg.pat),
                    _ => Err("unsupported arg type")?,
                })
                .collect::<Result<_>>()?;

            let method = &item.sig.ident;
            let ident = &self.data.ident;
            match self_ty {
                None => {
                    let trait_ = self.trait_path();
                    let arms = if trait_.is_none() {
                        self.arms(|v| quote!(#ident::#v(x) => x.#method(#(#args),*)))
                    } else {
                        self.arms(|v| quote!(#ident::#v(x) => #trait_::#method(x #(,#args)*)))
                    };
                    quote!(match self { #arms })
                }
                Some(SelfTypes::Pin(self_pin)) => {
                    self.unsafe_code = true;
                    let root = &self.root;
                    let pin = quote!(#root::pin::Pin);
                    let trait_ = self.trait_path();
                    let arms = if trait_.is_none() {
                        self.arms(
                            |v| quote!(#ident::#v(x) => #pin::new_unchecked(x).#method(#(#args),*)),
                        )
                    } else {
                        self.arms(|v| quote!(#ident::#v(x) => #trait_::#method(#pin::new_unchecked(x) #(,#args)*)))
                    };

                    match self_pin {
                        SelfPin::Ref => {
                            if self.unsafety || item.sig.unsafety.is_some() {
                                quote!(match #pin::get_ref(self) { #arms })
                            } else {
                                quote!(unsafe { match #pin::get_ref(self) { #arms } })
                            }
                        }
                        SelfPin::Mut => {
                            if self.unsafety || item.sig.unsafety.is_some() {
                                quote!(match #pin::get_mut_unchecked(self) { #arms })
                            } else {
                                quote!(unsafe { match #pin::get_mut_unchecked(self) { #arms } })
                            }
                        }
                    }
                }
            }
        };

        self.push_item(ImplItem::Method(method_from_method(
            item,
            block(vec![Stmt::Expr(syn::parse2(method)?)]),
        )));
        Ok(())
    }

    /// Append items from `ItemTrait`.
    ///
    /// The following items are ignored:
    /// - Generic associated types (GAT) (`TraitItem::Method` that has generics)
    /// - `TraitItem::Const`
    /// - `TraitItem::Macro`
    /// - `TraitItem::Verbatim`
    /// - `TraitItem::Method` that has the first argument other than `&self`, `&mut self`, `self` or `mut self`.
    pub(crate) fn append_items_from_trait(&mut self, item: ItemTrait) -> Result<()> {
        let fst = self.data.fields.iter().next();
        item.items.into_iter().try_for_each(|item| match item {
            TraitItem::Const(_) | TraitItem::Macro(_) | TraitItem::Verbatim(_) => Ok(()),
            TraitItem::Type(TraitItemType {
                ident, generics, ..
            }) => {
                // Generic associated types (GAT) are not supported
                if generics.params.is_empty() {
                    {
                        let trait_ = self.trait_.as_ref().map(|t| &t.ty);
                        syn::parse2(quote!(type #ident = <#fst as #trait_>::#ident;))
                    }
                    .map(|ty| self.push_item(ImplItem::Type(ty)))?;
                }
                Ok(())
            }
            TraitItem::Method(method) => {
                match method.sig.decl.inputs.iter().next() {
                    Some(FnArg::SelfRef(_)) | Some(FnArg::SelfValue(_)) => {}
                    _ => return Ok(()),
                }
                self.push_method(method)
            }
        })
    }

    /// impl_trait from `ItemTrait`.
    ///
    /// The following items are ignored:
    /// - Generic associated types (GAT) (`TraitItem::Method` that has generics)
    /// - `TraitItem::Const`
    /// - `TraitItem::Macro`
    /// - `TraitItem::Verbatim`
    /// - `TraitItem::Method` that has the first argument other than `&self`, `&mut self`, `self` or `mut self`.
    fn from_trait(
        data: &'a EnumData,
        root: TokenStream,
        trait_path: Path,
        items: Vec<ImplItem>,
        mut item: ItemTrait,
        supertraits_bounds: SmallVec<[Ident; 1]>,
    ) -> Result<Self> {
        fn generics_params<'a, I>(iter: I) -> impl Iterator<Item = Cow<'a, GenericParam>>
        where
            I: Iterator<Item = &'a GenericParam>,
        {
            iter.map(|param| match param {
                GenericParam::Type(ty) => Cow::Owned(GenericParam::Type(TypeParam {
                    attrs: ty.attrs.clone(),
                    ident: ty.ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                    eq_token: None,
                    default: None,
                })),
                param => Cow::Borrowed(param),
            })
        }

        let path = trait_path.clone();
        let mut generics = data.generics.clone();
        let trait_ = {
            if item.generics.params.is_empty() {
                path.clone()
            } else {
                let generics = generics_params(item.generics.params.iter());
                syn::parse2(quote!(#path<#(#generics),*>))?
            }
        };

        {
            let fst = data.fields.iter().next();
            let mut types: Stack<_> = item
                .items
                .iter()
                .filter_map(|item| match item {
                    TraitItem::Type(ty) => Some((false, &ty.ident)),
                    _ => None,
                })
                .collect();

            if item.supertraits.len() == 1 && !supertraits_bounds.is_empty() {
                types.extend(supertraits_bounds.iter().map(|ident| (true, ident)));
            }

            let where_clause = &mut generics.make_where_clause().predicates;
            where_clause.push(syn::parse2(quote!(#fst: #trait_))?);
            data.fields.iter().skip(1).try_for_each(|variant| {
                if types.is_empty() {
                    syn::parse2(quote!(#variant: #trait_)).map(|f| where_clause.push(f))
                } else {
                    let types = types.iter().map(|(supertraits, ident)| {
                        if *supertraits {
                            match item.supertraits.iter().next() {
                                Some(TypeParamBound::Trait(trait_)) => {
                                    quote!(#ident = <#fst as #trait_>::#ident)
                                }
                                _ => panic!("unsupported supertrait bound"),
                            }
                        } else {
                            quote!(#ident = <#fst as #trait_>::#ident)
                        }
                    });
                    if item.generics.params.is_empty() {
                        syn::parse2(quote!(#variant: #path<#(#types),*>))
                            .map(|f| where_clause.push(f))
                    } else {
                        let generics = generics_params(item.generics.params.iter());
                        syn::parse2(quote!(#variant: #path<#(#generics),*, #(#types),*>))
                            .map(|f| where_clause.push(f))
                    }
                }
            })?;
        }
        drop(supertraits_bounds);

        if !item.generics.params.is_empty() {
            mem::replace(&mut item.generics.params, Punctuated::new())
                .into_iter()
                .for_each(|param| generics.params.push(param));
        }

        if let Some(old) = &mut item.generics.where_clause {
            if !old.predicates.is_empty() {
                let where_clause = &mut generics.make_where_clause().predicates;
                mem::replace(&mut old.predicates, Punctuated::new())
                    .into_iter()
                    .for_each(|param| where_clause.push(param));
            }
        }

        let ident = &data.ident;
        let ty_generics = &data.generics;
        let mut impls = syn::parse2(quote!(#ident #ty_generics)).map(|self_ty| EnumImpl {
            data,
            defaultness: false,
            unsafety: item.unsafety.is_some(),
            generics,
            trait_: Some(Trait::new(trait_path, trait_)),
            self_ty: Box::new(self_ty),
            items,
            root,
            unsafe_code: false,
        })?;

        impls.append_items_from_trait(item).map(|_| impls)
    }

    pub(crate) fn build(self) -> TokenStream {
        self._build().into_token_stream()
    }

    fn _build(self) -> ItemImpl {
        ItemImpl {
            attrs: if self.unsafe_code {
                syn::parse2::<Attributes>(quote!(#[allow(unsafe_code)]))
                    .unwrap_or_else(|_| unreachable!())
                    .0
            } else {
                Vec::with_capacity(0)
            },
            defaultness: if self.defaultness {
                Some(default())
            } else {
                None
            },
            unsafety: if self.unsafety { Some(default()) } else { None },
            impl_token: default(),
            generics: self.generics,
            trait_: self.trait_.map(|Trait { ty, bang, .. }| {
                (if bang { Some(default()) } else { None }, ty, default())
            }),
            self_ty: self.self_ty,
            brace_token: default(),
            items: self.items,
        }
    }
}

fn method_from_method(method: TraitItemMethod, block: Block) -> ImplItemMethod {
    ImplItemMethod {
        attrs: method.attrs,
        vis: Visibility::Inherited,
        defaultness: None,
        sig: method.sig,
        block,
    }
}

#[derive(PartialEq, Eq)]
enum SelfTypes {
    /// `self: Pin<&Self>` or `self: Pin<&mut Self>`
    Pin(SelfPin),
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum SelfPin {
    /// `self: Pin<&Self>`
    Ref,
    /// `self: Pin<&mut Self>`
    Mut,
}

impl EnumData {
    pub(crate) fn parse(item: &ItemEnum) -> Result<Self> {
        let len = item.variants.len();
        if len < 2 {
            Err("cannot be implemented for enums with less than two variants")?;
        }

        let mut variants = Stack::with_capacity(len);
        let mut fields = Stack::with_capacity(len);
        for v in &item.variants {
            if v.discriminant.is_some() {
                Err("cannot be implemented for enums with discriminants")?;
            }

            match &v.fields {
                Fields::Unnamed(f) => match f.unnamed.len() {
                    1 => fields.push(f.unnamed.iter().next().unwrap().ty.clone()),
                    0 => Err("cannot be implemented for enums with zero fields")?,
                    _ => Err("cannot be implemented for enums with multiple fields")?,
                },
                Fields::Unit => Err("cannot be implemented for enums with units")?,
                Fields::Named(_) => Err("cannot be implemented for enums with named fields")?,
            }

            variants.push(v.ident.clone());
        }

        Ok(EnumData {
            ident: item.ident.clone(),
            generics: item.generics.clone(),
            variants,
            fields,
        })
    }
}
