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
   
   /// Takes a slice of (Attr, &str) as input. Loops through this slice
   /// and converts each member to a Val and pushes it into attributes Vec.
   pub fn attributes(mut self, values: &[Attr]) -> Self {
      for val in values {
         self.attributes.push(val.clone());
      }
      self
   }
   
   /// Takes a slice of (Prop, &str) as input. Loops through this slice
   /// and converts each member to a Val and pushes it into style Vec.
   pub fn style(mut self, values: &[Prop]) -> Self {
      for val in values {
         self.style.push(val.clone());
      }
      self
   }
   
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