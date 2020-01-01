use super::*;

mod attributes;
pub use attributes::*;

mod properties;
pub use properties::*;

mod tags;
pub use tags::*;

#[derive(Clone)]
pub struct El {
   is_text: bool,
   paired: bool,
   name: String,
   pub attributes: Vec<Attr>,
   pub style: Vec<Prop>,
   pub children: Vec<El>,
}

impl El {
   
   pub fn text<N: Into<String>>(text: N) -> Self {
      El {
         is_text: true,
         paired: false,
         name: text.into(),
         attributes: vec![],
         style: vec![],
         children: vec![],
      }
   }
   
   pub fn paired<N: Into<String>>(name: N, children: &[El]) -> Self {
      El {
         is_text: false,
         paired: true,
         name: name.into(),
         attributes: vec![],
         style: vec![],
         children: children.to_vec(),
      }
   }

   pub fn unpaired<N: Into<String>>(name: N) -> Self {
      El {
         is_text: false,
         paired: false,
         name: name.into(),
         attributes: vec![],
         style: vec![],
         children: vec![],
      }
   }
   
   pub fn attr(mut self, attr: Attr) -> Self {
      self.attributes.push(attr);
      self
   }
   
   pub fn add_style_prop(mut self, prop: Prop) -> Self {
      self.style.push(prop);
      self
   }
   
   pub fn add_child(&mut self, child: El) -> Result<(), ()> {
      
      if self.is_text || !self.paired {
         return Err(());
      }
      
      self.children.push(child);
      Ok(())
   }
   
   pub fn id_find(&self, id: &str) -> Option<&El> {
      
      for attr in self.attributes.iter() {
         if attr.name == "id" && attr.value == *id {
            return Some(self);
         }
      }
      
      if !self.is_text && self.paired {
         
         for child in self.children.iter() {
            
            let find = child.id_find(id);
            
            match find {
               Some(_) => { return find; },
               None => (),
            }
         }
      }
      
      None
   }
   
   pub fn format(&self, mut f: Formatter, depth: usize, make_pretty: bool) -> Formatter {
      
      if self.is_text {
         f.write(self.name.clone());
         return f;
      }
      
      if make_pretty {
         f.write("\n");
         f.prepend_depth(depth);
      }
      
      f.write("<");
      f.write(self.name.clone());
      
      if self.attributes.len() > 0 {
         f.write(self.attributes.iter().map(|a| format!("{}", a) ).collect::<Vec<String>>().join(" "));
      }
      
      if self.style.len() > 0 {
         f.write(" style='");
         f.write(self.style.iter().map(|p| format!("{}", p) ).collect::<Vec<String>>().join(" "));
         f.write("' ");
      }
      
      f.write(">");
      
      if self.paired {
         
         for child in self.children.iter() {
            f = child.format(f, depth+1, make_pretty);
         }
         
         if self.children.len() > 0 && make_pretty {
            f.write("\n");
            f.prepend_depth(depth);
         }
         
         f.write("</");
         f.write(self.name.clone());
         f.write(">");
         
      }
      
      f
   }
}