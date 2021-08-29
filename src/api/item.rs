use crate::{Attribute, Path, Span, Visibility};

#[derive(Clone, Debug)]
pub struct Mod {
    items: Vec<Item>,
    attrs: Vec<Attribute>,
    span: Span,
}

impl Mod {
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UseKind {
    Single,
    Glob,
    ListStem,
}

// TODO: fill in
#[derive(Clone, Debug)]
pub struct FnSig;

#[derive(Clone, Debug)]
pub struct Generics;

#[derive(Clone, Debug)]
pub struct Body;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ItemKind {
    Module(Mod),
    ExternCrate(Option<String>),
    Use(Path, UseKind),
    Function(FnSig, Generics, Body),
    TypeAlias(),
    Struct(),
    Enum(),
    Union(),
    Const(),
    Static(),
    Trait(),
    Impl(),
    ExternBlock(),
    Macro(),
}

#[derive(Clone, Debug)]
pub struct Item {
    kind: ItemKind,
    attr: Option<Attribute>,
    vis: Visibility,
    span: Span,
}

impl Item {
    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn attrs(&self) -> Option<&Attribute> {
        self.attr.as_ref()
    }

    pub fn vis(&self) -> &Visibility {
        &self.vis
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}
