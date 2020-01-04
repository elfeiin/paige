use paige::*;

fn main() {
   
   use Tag::*;
   use AttrName::*;
   use PropName::*;
   
   let page = Page::new(&[
      El::paired(Html, &[
         El::paired(Head, &[
            El::unpaired(Meta)
               .attributes(&[
                  (Charset, "UTF-8"),
               ]),
            El::paired(Style, &[])
         ]),
         El::paired(Body, &[])
      ])
   ]).format(false);
   
   println!("{}", page);
   
}