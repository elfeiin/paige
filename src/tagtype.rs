use super::*;

pub enum TagType {
   Paired(Vec<Markup>),
   Unpaired,
}