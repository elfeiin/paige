use super::*;

#[doc(hidden)]
pub struct Formatter {
   pub make_pretty: bool,
   pub buf: String,
}

impl Formatter {
   pub fn write<N: Into<String>>(&mut self, text: N) {
      self.buf = format!("{}{}", self.buf, text.into());
   }
   
   pub fn prepend_depth(&mut self, depth: usize) {
      if self.make_pretty {
         self.write("\n");
         for _ in 0..depth {
            self.write("   ");
         }
      }
   }
   
   pub fn parse_and_format(&mut self, quarrel: &Vec<El>, depth: usize) {
      for e in quarrel.iter() {
         if e.is_text {
            self.write(e.name.clone());
            continue;
         }
         
         self.prepend_depth(depth);
         
         self.write("<");
         self.write(e.name.clone());
         
         if e.attributes.len() > 0 || e.style.len() > 0 {
            self.write(" ");
         }
         
         if e.attributes.len() > 0 {
            self.write(e.attributes.iter().map(|a| format!("{}", a) ).collect::<Vec<String>>().join(" "));
         }
         
         if e.style.len() > 0 {
            self.write(" style='");
            self.write(e.style.iter().map(|p| format!("{}", p) ).collect::<Vec<String>>().join(" "));
            self.write("'");
         }
         
         if !e.paired {
            self.write("/");
         }
         
         self.write(">");
         
         if e.paired {
            
            self.parse_and_format(&e.content, depth + 1);
            
            if e.content.len() > 0 {
               self.prepend_depth(depth);
            }
            
            self.write("</");
            self.write(e.name.clone());
            self.write(">");
            
         }
      }
   }
   
   
}