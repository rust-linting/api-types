// Span and Id types are going to be tough/interesting, we need to keep the information in order to
// use rustc but it will tightly couple us to rustc's repr of id's and spans.

#[derive(Clone, Debug)]
pub struct Path;

#[derive(Clone, Debug)]
pub struct Span;
