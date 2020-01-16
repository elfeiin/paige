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
   El::paired(Tag::Div, &[
      
      El::paired(Tag::A, &[
         El::text("DDS")
      ])
      .attributes(&[
         (Attr::Href, "/")
      ]),
      
      El::text(" | "),
      
      El::paired(Tag::A, &[
         El::text("Search")
      ])
      .attributes(&[
         (Attr::Href, "")
      ]),
      
   ])
   .attributes(&[
      (Attr::Id, "navigation"),
      (Attr::Class, "nav"),
   ])
}

fn base_page() -> Page {
   Page::new(&[
      El::paired(Tag::Html, &[
         El::paired(Tag::Head, &[
            El::unpaired(Tag::Meta)
            .attributes(&[
               (Attr::Charset, "utf8"),
            ]),
            El::paired(Tag::Style, &[
               css(),
            ])
         ]),
         El::paired(Tag::Body, &[
            
            nav(),
            
            El::paired(Tag::Div, &[
               
            ])
            .attributes(&[
               (Attr::Id, "main"),
            ]),
            
         ])
      ])
   ])
}

fn index() -> Page {
   let mut page = base_page();
   
   page.children[0].children[1].children[1].children.push(
      El::text("Hello")
   );
   
   if let Some(el) = page.id_find("main") {
      el.children.push(
         El::text("<br>Hello")
      );
   }
   
   page
}

fn main() {
	println!("{}", index().format(true));
}