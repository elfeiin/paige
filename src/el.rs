use super::*;

#[derive(Clone)]
pub enum El {
   Text(String),
   Tag(Tag),
}

impl El {
   pub fn tag(tag: Tag) -> El {
      El::Tag(tag)
   }
   
   pub fn text<N: Into<String>>(text: N) -> Self {
      El::Text(text.into())
   }
   
   pub fn paired<N: Into<String>>(name: N, children: &[El]) -> Self {
      El::Tag(Tag::paired(name, children))
   }

   pub fn unpaired<N: Into<String>>(name: N) -> Self {
      El::Tag(Tag::unpaired(name))
   }
}

impl std::fmt::Display for El {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         El::Tag(el) => write!(f, "{}", el),
         El::Text(text) => write!(f, "{}", text)
      }
   }
}