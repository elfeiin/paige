/// A value struct used for storing attributes and style properties.
#[derive(Clone)]
pub struct Val {
   pub name: String,
   pub value: String,
}

impl Val {
   pub fn new<N: Into<String>, M: Into<String>>(name: N, value: M) -> Self {
      Val {
         name: name.into(),
         value: value.into(),
      }
   }
   
   pub fn value<N: Into<String>>(&mut self, value: N) {
      self.value = value.into();
   }
}