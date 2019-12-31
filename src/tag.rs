use super::*;

#[derive(Clone)]
pub struct Tag {
   name: String,
   paired: bool,
   attributes: Vec<Attr>,
   style: Vec<Prop>,
   children: Vec<El>,
}

impl Tag {
   pub fn paired<N: Into<String>>(name: N, children: &[El]) -> Tag {
      Tag {
         name: name.into(),
         paired: true,
         attributes: vec![],
         style: vec![],
         children: children.to_vec(),
      }
   }

   pub fn unpaired<N: Into<String>>(name: N) -> Tag {
      Tag {
         name: name.into(),
         paired: false,
         attributes: vec![],
         style: vec![],
         children: vec![],
      }
   }

   pub fn add_child(&mut self, child: El) {
      match self.paired {
         true => self.children.push(child),
         false => panic!("Unpaired tags cannot have children"),
      }
   }

   pub fn add_attribute(&mut self, attribute: Attr) {
      self.attributes.push(attribute);
   }

   pub fn add_style_prop(&mut self, prop: Prop) {
      self.style.push(prop);
   }
}

impl std::fmt::Display for Tag {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      
      write!(f, "\n<{}", self.name)?;
      
      if self.attributes.len() > 0 {
         write!(f, "{}", self.attributes.iter().map(|a| format!("{}", a) ).collect::<Vec<String>>().join(" "))?;
      }
      
      if self.style.len() > 0 {
         write!(f, "{}", self.style.iter().map(|p| format!("{}", p) ).collect::<Vec<String>>().join(" "))?;
      }
      
      write!(f, ">")?;
      
      if self.paired {
         
         for child in self.children.iter() {
            write!(f, "{}", child)?;
         }
         
         if self.children.len() > 0 {
            write!(f, "\n")?;
         }
         
         write!(f, "</{}>", self.name)?;
         
      }
      
      Ok(())
      
   }
}

pub enum TagName {
   A,
   Abbr,
   Acronym,
   Address,
   Applet,
   Area,
   Article,
   Aside,
   Audio,
   B,
   Base,
   Basefont,
   Bdi,
   Bdo,
   Big,
   Blockquote,
   Body,
   Br,
   Button,
   Canvas,
   Caption,
   Center,
   Cite,
   Code,
   Col,
   Colgroup,
   Data,
   Datalist,
   Dd,
   Del,
   Details,
   Dfn,
   Dialog,
   Dir,
   Div,
   Dl,
   Dt,
   Em,
   Embed,
   Fieldset,
   FigCaption,
   Figure,
   Font,
   Footer,
   Form,
   Frame,
   Frameset,
   H1,
   Head,
   Header,
   Hr,
   Html,
   I,
   IFrame,
   Img,
   Input,
   Ins,
   Kbd,
   Label,
   Legend,
   Li,
   Link,
   Main,
   Map,
   Mark,
   Meta,
   Meter,
   Nav,
   Noframes,
   Noscript,
   Object,
   Ol,
   Optgroup,
   Option_,
   Output,
   P,
   Param,
   Picture,
   Pre,
   Progress,
   Q,
   Rp,
   Rt,
   Ruby,
   S,
   Samp,
   Script,
   Section,
   Select,
   Small,
   Source,
   Span,
   Strike,
   Strong,
   Style,
   Sub,
   Summary,
   Sup,
   Svg,
   Table,
   Tbody,
   Td,
   Template,
   TextArea,
   Tfoot,
   Th,
   Thead,
   Time,
   Title,
   Tr,
   Track,
   Tt,
   U,
   Ul,
   Var,
   Video,
   Wbr,
}

impl std::convert::Into<String> for TagName {
   fn into(self) -> String {
      match self {
         TagName::A => "a",
         TagName::Abbr => "abbr",
         TagName::Acronym => "acronym",
         TagName::Address => "address",
         TagName::Applet => "applet",
         TagName::Area => "area",
         TagName::Article => "article",
         TagName::Aside => "aside",
         TagName::Audio => "audio",
         TagName::B => "b",
         TagName::Base => "base",
         TagName::Basefont => "basefont",
         TagName::Bdi => "bdi",
         TagName::Bdo => "bdo",
         TagName::Big => "big",
         TagName::Blockquote => "blockquote",
         TagName::Body => "body",
         TagName::Br => "br",
         TagName::Button => "button",
         TagName::Canvas => "canvas",
         TagName::Caption => "caption",
         TagName::Center => "center",
         TagName::Cite => "cite",
         TagName::Code => "code",
         TagName::Col => "col",
         TagName::Colgroup => "colgroup",
         TagName::Data => "data",
         TagName::Datalist => "datalist",
         TagName::Dd => "dd",
         TagName::Del => "del",
         TagName::Details => "details",
         TagName::Dfn => "dfn",
         TagName::Dialog => "dialog",
         TagName::Dir => "dir",
         TagName::Div => "div",
         TagName::Dl => "dl",
         TagName::Dt => "dt",
         TagName::Em => "em",
         TagName::Embed => "embed",
         TagName::Fieldset => "fieldset",
         TagName::FigCaption => "figcaption",
         TagName::Figure => "figure",
         TagName::Font => "font",
         TagName::Footer => "footer",
         TagName::Form => "form",
         TagName::Frame => "frame",
         TagName::Frameset => "frameset",
         TagName::H1 => "h1",
         TagName::Head => "head",
         TagName::Header => "header",
         TagName::Hr => "hr",
         TagName::Html => "html",
         TagName::I => "i",
         TagName::IFrame => "iframe",
         TagName::Img => "img",
         TagName::Input => "input",
         TagName::Ins => "ins",
         TagName::Kbd => "kbd",
         TagName::Label => "label",
         TagName::Legend => "legend",
         TagName::Li => "li",
         TagName::Link => "link",
         TagName::Main => "main",
         TagName::Map => "map",
         TagName::Mark => "mark",
         TagName::Meta => "meta",
         TagName::Meter => "meter",
         TagName::Nav => "nav",
         TagName::Noframes => "noframes",
         TagName::Noscript => "noscript",
         TagName::Object => "object",
         TagName::Ol => "ol",
         TagName::Optgroup => "optgroup",
         TagName::Option_ => "option",
         TagName::Output => "output",
         TagName::P => "p",
         TagName::Param => "param",
         TagName::Picture => "picture",
         TagName::Pre => "pre",
         TagName::Progress => "progress",
         TagName::Q => "q",
         TagName::Rp => "rp",
         TagName::Rt => "rt",
         TagName::Ruby => "ruby",
         TagName::S => "s",
         TagName::Samp => "samp",
         TagName::Script => "script",
         TagName::Section => "section",
         TagName::Select => "select",
         TagName::Small => "small",
         TagName::Source => "source",
         TagName::Span => "span",
         TagName::Strike => "strike",
         TagName::Strong => "strong",
         TagName::Style => "style",
         TagName::Sub => "sub",
         TagName::Summary => "summary",
         TagName::Sup => "sup",
         TagName::Svg => "svg",
         TagName::Table => "table",
         TagName::Tbody => "tbody",
         TagName::Td => "td",
         TagName::Template => "template",
         TagName::TextArea => "textarea",
         TagName::Tfoot => "tfoot",
         TagName::Th => "th",
         TagName::Thead => "thead",
         TagName::Time => "time",
         TagName::Title => "title",
         TagName::Tr => "tr",
         TagName::Track => "track",
         TagName::Tt => "tt",
         TagName::U => "u",
         TagName::Ul => "ul",
         TagName::Var => "var",
         TagName::Video => "video",
         TagName::Wbr => "wbr",
      }.into()
   }
}