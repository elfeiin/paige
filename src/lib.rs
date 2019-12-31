#![allow(dead_code, non_camel_case_types)]

mod property;
use property::*;

mod attribute;
use attribute::*;

mod tag;
use tag::*;

mod traits;
use traits::*;

mod markup;
use markup::*;

pub struct Page {
   children: Vec<Markup>,
}

impl IsHtmlTag for Page {
   fn add(&mut self, child: Markup) {
      self.children.push(child);
   }
}

impl Page {
   pub fn new() -> Self {
      Page {
         children: vec!(),
      }
   }
}

#[cfg(test)]
mod tests {
   
   use super::*;
   
   #[test]
   fn it_works() {
      
      // TODO: Write test that makes a basic html page.
      
      let page = Page::new();
      
      
      
   }
}
