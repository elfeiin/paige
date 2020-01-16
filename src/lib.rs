#![allow(dead_code, unused_imports)]

/// A simple HTML templating API.
/// 
/// # Examples
/// 
/// ```
///  example
/// ```
/// 

// mod element;
// pub use element::*;

mod element;
pub use element::*;

/// pagey
mod page;
pub use page::*;

mod formatter;
pub use formatter::*;

impl Format for Page {
   fn fmt(&self, mut f: Formatter) -> Formatter {
      
      f.write("<!DOCTYPE HTML>");
      
      for child in self.content.iter() {
         f = child.fmt(f);
      }
      
      f
   }
}

impl Format for El {
   /// Takes local Formatter, current indentation depth, and whether should
   /// output pretty, writes to the Formatter, and returns the Formatter.
   /// This controls how elements are displayed.
   fn fmt(&self, mut f: Formatter) -> Formatter {
      
      f.depth = f.depth + 1;
      
      if self.is_text {
         f.write(self.name.clone());
         return f;
      }
      
      f.prepend_depth();
      
      f.write("<");
      f.write(self.name.clone());
      
      if self.attributes.len() > 0 || self.style.len() > 0 {
         f.write(" ");
      }
      
      if self.attributes.len() > 0 {
         f.write(self.attributes.iter().map(|a| format!("{}", a) ).collect::<Vec<String>>().join(" "));
      }
      
      if self.style.len() > 0 {
         f.write(" style='");
         f.write(self.style.iter().map(|p| format!("{}", p) ).collect::<Vec<String>>().join(" "));
         f.write("'");
      }
      
      if !self.paired {
         f.write("/");
      }
      
      f.write(">");
      
      if self.paired {
         
         for child in self.content.iter() {
            f = child.fmt(f);
         }
         
         if self.content.len() > 0 {
            f.prepend_depth();
         }
         
         f.write("</");
         f.write(self.name.clone());
         f.write(">");
         
      }
      
      f.depth = f.depth - 1;
      
      f
   }
}