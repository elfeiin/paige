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
   
   pub fn add(mut self, child: El) -> Self {
      self.children.push(child);
      self
   }
   
   pub fn id_find(&self, id: &String) -> Option<&El> {
      
      for child in self.children.iter() {
         
         let find = child.id_find(id);
         
         match find {
            Some(_) => { return find; },
            None => (),
         }
      }
      
      None
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

#[cfg(test)]
mod tests {
   
   use super::*;
   
   #[test]
   fn test_add_and_new() {
      
      let page = Page::new(&[]);
      
      let html = El::paired(Tag::Html, &[]);
      
      page.add(html);
      
      
      
   }
}

impl std::fmt::Display for Page {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      
      write!(f, "{}", self.format(true))?;
      
      Ok(())
      
   }
}