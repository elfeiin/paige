#![allow(dead_code, non_camel_case_types)]

mod property;
use property::*;

mod attribute;
use attribute::*;

mod tag;
use tag::*;

mod traits;
use traits::*;

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
