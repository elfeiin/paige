use paige::*;

fn main() {
   
   use Tag::*;
   use Prop::*;
   
   let page = Page::new(&[
      El::paired(Html, &[
         El::paired(Head, &[
            El::unpaired(Meta)
            .attributes(&[
               (Attr::Charset, "UTF-8"),
            ]),
            El::paired(Style, &[])
         ]),
         El::paired(Body, &[
            
         ])
         .style(&[
            (Background, "green")
         ])
      ])
   ]).format(false);
   
   println!("{}", page);
   
}