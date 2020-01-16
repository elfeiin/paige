#[doc(hidden)]
pub struct Formatter {
   pub depth: usize,
   pub make_pretty: bool,
   pub buf: String,
}

impl Formatter {
   pub fn write<N: Into<String>>(&mut self, text: N) {
      self.buf = format!("{}{}", self.buf, text.into());
   }
   
   pub fn prepend_depth(&mut self) {
      if self.make_pretty {
         self.write("\n");
         for _ in 0..self.depth {
            self.write("   ");
         }
      }
   }
}

pub trait Format {
   fn fmt(&self, f: Formatter) -> Formatter;
}