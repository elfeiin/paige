use super::*;

pub struct Page {
   children: Vec<Markup>,
}

impl Page {
   pub fn new() -> Self {
      Page {
         children: vec!(),
      }
   }
   
   fn add(&mut self, child: Markup) {
      self.children.push(child);
   }
}