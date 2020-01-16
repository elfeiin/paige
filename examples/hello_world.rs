use paige::*;

fn main() {
   
   let page = Page::new(&[
      Tag::Html.content(&[
         Tag::Head.content(&[
            Tag::Meta.attributes(&[Attr::Charset("UTF-8".into())])
         ]),
         Tag::Body.style(&[Prop::Background("green".into())]).content(&[])
      ])
   ]);
   
   println!("{}", page.format(false));
   println!("");
   println!("{}", page.format(true));
   
}