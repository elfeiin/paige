use super::*;

pub struct Page {
   children: Vec<El>,
}

impl Page {
   pub fn new(children: &[El]) -> Self {
      Page {
         children: children.to_vec(),
      }
   }
   
   fn add(&mut self, child: El) {
      self.children.push(child);
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