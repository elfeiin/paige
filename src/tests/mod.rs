use super::*;

#[test]
fn print_bare_html_tag() {
   
   use TagName::*;
   
   let page = Page::new(
      &[
         
         El::paired(Html, &[
            
            
            
         ])
      ]
   );
   
   println!("{}", page);
   
}