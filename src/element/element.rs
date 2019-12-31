use super::*;

#[derive(Clone)]
pub struct Element {
   name: String,
   paired: bool,
   attributes: Vec<Attr>,
   style: Vec<Prop>,
   children: Vec<Up>,
}

impl Element {
   pub fn paired<N: Into<String>>(name: N, children: &[Up]) -> Element {
      Element {
         name: name.into(),
         paired: true,
         attributes: vec![],
         style: vec![],
         children: children.to_vec(),
      }
   }

   pub fn unpaired<N: Into<String>>(name: N) -> Element {
      Element {
         name: name.into(),
         paired: false,
         attributes: vec![],
         style: vec![],
         children: vec![],
      }
   }

   pub fn add_child(&mut self, child: Up) {
      match self.paired {
         true => self.children.push(child),
         false => panic!("Unpaired tags cannot have children"),
      }
   }

   pub fn add_attribute(&mut self, attribute: Attr) {
      self.attributes.push(attribute);
   }

   pub fn add_style_prop(&mut self, prop: Prop) {
      self.style.push(prop);
   }
   
   pub fn format(&self, mut f: Formatter, depth: usize, make_pretty: bool) -> Formatter {
      
      f.write("\n");
      
      if make_pretty {
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
         
         if self.children.len() > 0 {
            
            f.write("\n");
            
            if make_pretty {
               f.prepend_depth(depth);
            }
         }
         
         f.write("</");
         f.write(self.name.clone());
         f.write(">");
         
      }
      
      f
   }
   
}

impl std::fmt::Display for Element {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      
      write!(f, "\n<{}", self.name)?;
      
      if self.attributes.len() > 0 {
         write!(f, "{}", self.attributes.iter().map(|a| format!("{}", a) ).collect::<Vec<String>>().join(" "))?;
      }
      
      if self.style.len() > 0 {
         write!(f, "{}", self.style.iter().map(|p| format!("{}", p) ).collect::<Vec<String>>().join(" "))?;
      }
      
      write!(f, ">")?;
      
      if self.paired {
         
         for child in self.children.iter() {
            write!(f, "{}", child)?;
         }
         
         if self.children.len() > 0 {
            write!(f, "\n")?;
         }
         
         write!(f, "</{}>", self.name)?;
         
      }
      
      Ok(())
      
   }
}