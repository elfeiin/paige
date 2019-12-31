use super::*;

pub struct Page {
   children: Vec<Up>,
}

impl Page {
   pub fn new(children: &[Up]) -> Self {
      Page {
         children: children.to_vec(),
      }
   }
   
   pub fn add(&mut self, child: Up) {
      self.children.push(child);
   }
   
   pub fn format(make_pretty: bool) -> String {
      if make_pretty {
         String::new()
      } else {
         String::new()
      }
   }
   
}

impl std::fmt::Display for Page {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      
      write!(f, "<!DOCTYPE HTML>")?;
      
      for child in self.children.iter() {
         write!(f, "{}", child)?;
      }
      
      Ok(())
      
   }
}