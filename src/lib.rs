#![allow(dead_code, non_camel_case_types)]

mod property;
use property::*;

mod attribute;
use attribute::*;

mod tag;
use tag::*;

mod traits;
use traits::*;

pub enum Markup {
   Tag(Tag),
   Text(String)
}

impl std::fmt::Display for Markup {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         Markup::Tag(el) => write!(f, "{}", el),
         Markup::Text(text) => write!(f, "{}", text)
      }
   }
}

pub enum TagType {
   Paired(Vec<Markup>),
   Unpaired,
}

pub struct Page {
   markups: TagType,
}

// TODO: impl IsHtmlTag for Page.

impl Page {
   pub fn new() -> Self {
      Page {
         markups: TagType::Unpaired,
      }
   }
}

#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
      
      // TODO: Write test that makes a basic html page.
      
   }
}
