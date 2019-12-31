#![allow(dead_code, non_camel_case_types, unused_imports)]

mod props;
use props::*;

mod attrs;
use attrs::*;

mod tagtype;
use tagtype::*;

mod tag;
use tag::*;

mod markup;
use markup::*;

mod page;
use page::*;

#[cfg(test)]
mod tests;