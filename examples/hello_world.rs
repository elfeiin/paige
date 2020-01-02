use paige::*;

fn main() {
   
   use Tag::*;
   
   let page = Page::new(&[
      El::paired(Html, &[
         El::paired(Head, &[
            El::unpaired(Meta)
               .attr(AttrName::Charset, "UTF-8"),
            El::paired(Style, &[
               
            ])
         ]),
         El::paired(Body, &[
            
         ])
      ])
   ]).format(true);
   
   println!("{}", page);
   
}