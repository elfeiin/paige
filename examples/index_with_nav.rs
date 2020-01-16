use paige::*;

fn css() -> El {
   El::text(
"* {
	margin: 0 0 0 0;
	background: #12121225;
	border: 1px solid #333;
	color: #999;
}

html {
	height: 100%;
	border: none;
}

body {
	width: 100%;
	height: 100%;
	position: fixed;
	background: linear-gradient(
		45deg,
		#27232f,
		#000 45%,
		#27232f
	);
	border: none;
}

	#navigation {
		margin: 1cm 0 0 2cm;
		width: 14cm;
		height: 1cm;
	}
	
		a {
			border: none;
		}
	
	#main {
		margin: 1cm 1.5cm 1.5cm 2cm;
		width: auto;
		height: auto;
	}
	
.nav {
	line-height: 1cm;
}")
}

fn nav() -> El {
   Tag::Div
   .attributes(&[
      Attr::Id("navigation".into()),
      Attr::Class("nav".into()),
   ])
   .content(&[
      Tag::A
      .attributes(&[
         Attr::Href("/".into()),
      ])
      .content(&[
         El::text("DDS")
      ]),
      El::text(" | "),
      Tag::A
      .attributes(&[
         Attr::Href("".into()),
      ])
      .content(&[
         El::text("Search")
      ]),
   ])
}

fn base_page() -> Page {
   Page::new(&[
      Tag::Html
      .content(&[
         Tag::Head
         .content(&[
            Tag::Meta
            .attributes(&[
               Attr::Charset("UTF-8".into()),
            ]),
            Tag::Style.content(&[css()])
         ]),
         Tag::Body
         .content(&[
            nav(),
            Tag::Div.attributes(&[Attr::Id("main".into())]),
         ])
      ])
   ])
}

fn index() -> Page {
   let mut page = base_page();
   
   page.content[0].content[1].content[1].content.push(
      El::text("Hello")
   );
   
   if let Some(el) = page.id_find("main") {
      el.content.push(
         El::text("<br>Hello")
      );
   }
   
   page
}

fn main() {
	println!("{}", index().format(true));
}