use super::*;

pub struct Page {
   children: Vec<Element>,
}

impl Page {
   pub fn new() -> Self {
      Page {
         children: vec!(),
      }
   }
   
   fn add(&mut self, child: Element) {
      self.children.push(child);
   }
}