
pub struct Formatter {
   pub buf: String,
}

impl Formatter {
   pub fn write<N: Into<String>>(&mut self, text: N) {
      self.buf = format!("{}{}", self.buf, text.into());
   }
   
   pub fn prepend_depth(&mut self, depth: usize) {
      for _ in 0..depth {
         self.write("   ");
      }
   }
}