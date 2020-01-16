mod attributes;
pub use attributes::*;

mod properties;
pub use properties::*;

mod tags;
pub use tags::*;

/// An HTML element.
#[derive(Clone)]
pub struct El {
   pub name: String,
   pub is_text: bool,
   pub paired: bool,
   pub attributes: Vec<Attr>,
   pub style: Vec<Prop>,
   pub content: Vec<El>,
}

impl El {
   
   /// Creates a new element that displays its name as text and nothing else
   /// when formatted. This is useful for making innerHTML something other
   /// than a tag, such as inside the <style> or <script> tags.
   pub fn text<N: Into<String>>(text: N) -> Self {
      El {
         is_text: true,
         paired: false,
         name: text.into(),
         attributes: vec![],
         style: vec![],
         content: vec![],
      }
   }
   
   /// Takes a slice of Attr as input and pushes each member onto .attributes vec.
   pub fn attributes(mut self, values: &[Attr]) -> Self {
      for val in values {
         self.attributes.push(val.clone());
      }
      self
   }
   
   /// Takes a slice of Prop as input and pushes each member onto .style vec.
   pub fn style(mut self, values: &[Prop]) -> Self {
      for val in values {
         self.style.push(val.clone());
      }
      self
   }
   
   /// Takes a slice of El and pushes each element onto .content.
   pub fn content(mut self, values: &[El]) -> Self {
      for val in values {
         self.content.push(val.clone());
      }
      self
   }
   
   /// Allows for finding a child element by its id attribute.
   pub fn id_find(&mut self, id: &str) -> Option<&mut El> {
      
      for attr in self.attributes.iter() {
         if let Attr::Id(val) = attr {
            if *val == *id {
               return Some(self);
            }
         }
      }
      
      if !self.is_text && self.paired {
         
         for child in self.content.iter_mut() {
            
            let find = child.id_find(id);
            
            match find {
               Some(_) => { return find; },
               None => (),
            }
         }
      }
      
      None
   }
   
}