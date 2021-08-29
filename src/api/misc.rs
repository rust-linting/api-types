use crate::api::common::Path;
use crate::Span;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Visibility {
    Public,
    Crate,
    Restricted,
    Inherited,
}

#[derive(Clone, Debug)]
pub struct TokenTree;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DelimTokenTree {
    Paren(TokenTree),
    Brace(TokenTree),
    Bracket(TokenTree),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum MacroInvocation {
    NoSemi(DelimTokenTree),
    Semi(DelimTokenTree),
}

// TODO: according to https://doc.rust-lang.org/stable/reference/macros-by-example.html
#[derive(Clone, Debug)]
pub struct MacroRules;

// TODO: So here is one of the big differences between AST and HIR (early/late) the only macro
// HIR has is `macro_rules` definitions there is no such thing as a proc-macro or a macro call site
// since they are all expanded

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum MacroItem {
    Invocation(MacroInvocation),
    MacroRulesDef(MacroRules),
}

#[derive(Clone, Debug)]
pub struct Attr {
    path: Path,
    input: Option<DelimTokenTree>,
    span: Span,
    // id: SomeIdKind,
}

impl Attr {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn input(&self) -> Option<&DelimTokenTree> {
        self.input.as_ref()
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Attribute {
    Inner(Attr),
    Outer(Attr),
}
