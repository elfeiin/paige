use super::*;

#[derive(Clone)]
pub enum Element {
   Text(String),
   Tag(HtmlTag),
}

impl Element {
   pub fn text(text: String) -> Element {
      Element::Text(text)
   }

   pub fn tag(tag: HtmlTag) -> Element {
      Element::Tag(tag)
   }
}

impl std::fmt::Display for Element {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         Element::Tag(el) => write!(f, "{}", el),
         Element::Text(text) => write!(f, "{}", text)
      }
   }
}