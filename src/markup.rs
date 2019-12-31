use super::*;

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