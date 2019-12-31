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
   
   pub fn format(&self, make_pretty: bool) -> String {
      
      let mut f = Formatter {
         buf: String::new(),
      };
      
      f.write("<!DOCTYPE HTML>");
      
      for child in self.children.iter() {
         f = child.format(f, 0, make_pretty);
      }
      
      f.buf
      
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