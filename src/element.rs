use super::*;

/// An HTML element.
#[derive(Clone)]
pub struct El {
   pub is_text: bool,
   pub paired: bool,
   pub name: String,
   pub attributes: Vec<Val>,
   pub style: Vec<Val>,
   pub children: Vec<El>,
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
         children: vec![],
      }
   }
   
   /// Creates an element that displays an open tag, a closing tag, and
   /// whatever attributes/style/children are associated with this element
   /// when formatted.
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
   
   /// Same as paired, but this tag does not display children and has no
   /// closing tag, only '/>'.
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
   
   /// Takes a slice of (Attr, &str) as input. Loops through this slice
   /// and converts each member to a Val and pushes it into attributes Vec.
   pub fn attributes(mut self, values: &[(Attr, &str)]) -> Self {
      for val in values {
         self.attributes.push(Val::new(val.0.clone(), val.1));
      }
      self
   }
   
   /// Takes a slice of (Prop, &str) as input. Loops through this slice
   /// and converts each member to a Val and pushes it into style Vec.
   pub fn style(mut self, values: &[(Prop, &str)]) -> Self {
      for val in values {
         self.style.push(Val::new(val.0.clone(), val.1));
      }
      self
   }
   
   /// Allows for finding a child element by its id attribute.
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
   
   /// Takes local Formatter, current indentation depth, and whether should
   /// output pretty, writes to the Formatter, and returns the Formatter.
   /// This controls how elements are displayed.
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
      
      if self.attributes.len() > 0 || self.style.len() > 0 {
         f.write(" ");
      }
      
      if self.attributes.len() > 0 {
         f.write(self.attributes.iter().map(|a| format!("{}='{}'", a.name, a.value) ).collect::<Vec<String>>().join(" "));
      }
      
      if self.style.len() > 0 {
         f.write(" style='");
         f.write(self.style.iter().map(|p| format!("{}: {};", p.name, p.value) ).collect::<Vec<String>>().join(" "));
         f.write("'");
      }
      
      if !self.paired {
         f.write("/");
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