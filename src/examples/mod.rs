use super::*;

#[test]
fn print_bare_html_tag() {
   
   use Tag::*;
   
   let page = Page::new(
      &[
         
         Up::paired(Html, &[])
         
      ]
   );
   
   println!("{}", page.format(true));
   
}

#[test]
fn print_bare_page() {
   
   use Tag::*;
   
   let page = Page::new(
      &[
         
         Up::paired(Html, &[
            
            Up::paired(Head, &[
               
               Up::paired(Style, &[
                  
                  
                  
               ])
               
            ]),
            
            Up::paired(Body, &[
               
               
               
            ]),
            
         ])
      ]
   );
   
   println!("{}", page.format(true));
   
}