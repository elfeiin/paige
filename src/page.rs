use super::*;

/// The Page struct.
/// Stores a list of nested HTML elements.
pub struct Page {
   pub content: Vec<El>,
}

impl Page {
   /// Returns a new Page. Takes a slice of El as input.
   pub fn new(content: &[El]) -> Self {
      Page {
         content: content.to_vec(),
      }
   }
   
   /// Pushes an El onto content field vec.
   pub fn add(mut self, child: El) -> Self {
      self.content.push(child);
      self
   }
   
   /// Allows for finding a child element by its id attribute.
   pub fn id_find(&mut self, id: &str) -> Option<&mut El> {
      
      for child in self.content.iter_mut() {
         
         let find = child.id_find(id);
         
         match find {
            Some(_) => { return find; },
            None => (),
         }
      }
      
      None
   }
   
   /// Formats the Page for display or storage. Automatically
   /// prepends '<!DOCTYPE HTML> to the beginning of the file.
   pub fn format(&self, make_pretty: bool) -> String {
      let mut f = Formatter {
         make_pretty,
         buf: String::new(),
      };
      
      f.parse_and_format(&self.content, 0);
      
      f.buf
   }
}

impl std::fmt::Display for Page {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.format(true))
   }
}