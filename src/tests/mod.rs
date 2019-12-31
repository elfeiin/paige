use super::*;

#[test]
fn print_bare_html_tag() {
   
   use Tag::*;
   
   let page = Page::new(
      &[
         
         Up::paired(Html, &[
            
            Up::paired(Head, &[
               
               
               
            ])
            
         ])
      ]
   );
   
   println!("{}", page);
   
}