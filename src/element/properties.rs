/// CSS properties
#[derive(Clone)]
pub enum Prop {
   AlignContent(String),
   AlignItems(String),
   AlignSelf(String),
   All(String),
   Animation(String),
   AnimationDelay(String),
   AnimationDirection(String),
   AnimationDuration(String),
   AnimationFillMode(String),
   AnimationIterationCount(String),
   AnimationName(String),
   AnimationPlayState(String),
   AnimationTimingFunction(String),
   BackfaceVisibility(String),
   Background(String),
   BackgroundAttachment(String),
   BackgroundBlendMode(String),
   BackgroundClip(String),
   BackgroundColor(String),
   BackgroundImage(String),
   BackgroundOrigin(String),
   BackgroundPosition(String),
   BackgroundRepeat(String),
   BackgroundSize(String),
   Border(String),
   BorderBottom(String),
   BorderBottomColor(String),
   BorderBottomLeftRadius(String),
   BorderBottomRightRadius(String),
   BorderBottomStyle(String),
   BorderBottomWidth(String),
   BorderCollapse(String),
   BorderColor(String),
   BorderImage(String),
   BorderImageOutset(String),
   BorderImageRepeat(String),
   BorderImageSlice(String),
   BorderImageSource(String),
   BorderImageWidth(String),
   BorderLeft(String),
   BorderLeftColor(String),
   BorderLeftStyle(String),
   BorderLeftWidth(String),
   BorderRadius(String),
   BorderRight(String),
   BorderRightColor(String),
   BorderRightStyle(String),
   BorderRightWidth(String),
   BorderSpacing(String),
   BorderStyle(String),
   BorderTop(String),
   BorderTopColor(String),
   BorderTopLeftRadius(String),
   BorderTopRightRadius(String),
   BorderTopStyle(String),
   BorderTopWidth(String),
   BorderWidth(String),
   Bottom(String),
   BoxDecorationBreak(String),
   BoxShadow(String),
   BoxSizing(String),
   BreakAfter(String),
   BreakBefore(String),
   BreakInside(String),
   CaptionSide(String),
   CaretColor(String),
   Clear(String),
   Clip(String),
   Color(String),
   ColumnCount(String),
   ColumnFill(String),
   ColumnGap(String),
   ColumnRule(String),
   ColumnRuleColor(String),
   ColumnRuleStyle(String),
   ColumnRuleWidth(String),
   ColumnSpan(String),
   ColumnWidth(String),
   Columns(String),
   Content(String),
   CounterIncrement(String),
   CounterReset(String),
   Cursor(String),
   Direction(String),
   Display(String),
   EmptyCells(String),
   Filter(String),
   Flex(String),
   FlexBasis(String),
   FlexDirection(String),
   FlexFlow(String),
   FlexGrow(String),
   FlexShrink(String),
   FlexWrap(String),
   Float(String),
   Font(String),
   FontFamily(String),
   FontFeatureSettings(String),
   FontKerning(String),
   FontLanguageOverride(String),
   FontSize(String),
   FontSizeAdjust(String),
   FontStretch(String),
   FontStyle(String),
   FontSynthesis(String),
   FontVariant(String),
   FontVariantAlternates(String),
   FontVariantCaps(String),
   FontVariantEastAsian(String),
   FontVariantLigatures(String),
   FontVariantNumeric(String),
   FontVariantPosition(String),
   FontWeight(String),
   Grid(String),
   GridArea(String),
   GridAutoColumns(String),
   GridAutoFlow(String),
   GridAutoRows(String),
   GridColumn(String),
   GridColumnEnd(String),
   GridColumnGap(String),
   GridColumnStart(String),
   GridGap(String),
   GridRow(String),
   GridRowEnd(String),
   GridRowGap(String),
   GridRowStart(String),
   GridTemplate(String),
   GridTemplateAreas(String),
   GridTemplateColumns(String),
   GridTemplateRows(String),
   HangingPunctuation(String),
   Height(String),
   Hyphens(String),
   ImageRendering(String),
   Isolation(String),
   JustifyContent(String),
   Left(String),
   LetterSpacing(String),
   LineBreak(String),
   LineHeight(String),
   ListStyle(String),
   ListStyleImage(String),
   ListStylePosition(String),
   ListStyleType(String),
   Margin(String),
   MarginBottom(String),
   MarginLeft(String),
   MarginRight(String),
   MarginTop(String),
   MaxHeight(String),
   MaxWidth(String),
   MinHeight(String),
   MinWidth(String),
   MixBlendMode(String),
   ObjectFit(String),
   ObjectPosition(String),
   Opacity(String),
   Order(String),
   Orphans(String),
   Outline(String),
   OutlineColor(String),
   OutlineOffset(String),
   OutlineStyle(String),
   OutlineWidth(String),
   Overflow(String),
   OverflowWrap(String),
   OverflowX(String),
   OverflowY(String),
   Padding(String),
   PaddingBottom(String),
   PaddingLeft(String),
   PaddingRight(String),
   PaddingTop(String),
   PageBreakAfter(String),
   PageBreakBefore(String),
   PageBreakInside(String),
   Perspective(String),
   PerspectiveOrigin(String),
   PointerEvents(String),
   Position(String),
   Quotes(String),
   Resize(String),
   Right(String),
   ScrollBehavior(String),
   TabSize(String),
   TableLayout(String),
   TextAlign(String),
   TextAlignLast(String),
   TextCombineUpright(String),
   TextDecoration(String),
   TextDecorationColor(String),
   TextDecorationLine(String),
   TextDecorationStyle(String),
   TextIndent(String),
   TextJustify(String),
   TextOrientation(String),
   TextOverflow(String),
   TextShadow(String),
   TextTransform(String),
   TextUnderlinePosition(String),
   Top(String),
   Transform(String),
   TransformOrigin(String),
   TransformStyle(String),
   Transition(String),
   TransitionDelay(String),
   TransitionDuration(String),
   TransitionProperty(String),
   TransitionTimingFunction(String),
   UnicodeBidi(String),
   UserSelect(String),
   VerticalAlign(String),
   Visibility(String),
   WhiteSpace(String),
   Widows(String),
   Width(String),
   WordBreak(String),
   WordSpacing(String),
   WordWrap(String),
   WritingMode(String),
   ZIndex(String),
}

impl Prop {
   pub fn into_string(&self) -> String {
      match self {
         Prop::AlignContent(val) => format!("align-content: {};", val),
         Prop::AlignItems(val) => format!("align-items: {};", val),
         Prop::AlignSelf(val) => format!("align-self: {};", val),
         Prop::All(val) => format!("all: {};", val),
         Prop::Animation(val) => format!("animation: {};", val),
         Prop::AnimationDelay(val) => format!("animation-delay: {};", val),
         Prop::AnimationDirection(val) => format!("animation-direction: {};", val),
         Prop::AnimationDuration(val) => format!("animation-duration: {};", val),
         Prop::AnimationFillMode(val) => format!("animation-fill-mode: {};", val),
         Prop::AnimationIterationCount(val) => format!("animation-iteration-count: {};", val),
         Prop::AnimationName(val) => format!("animation-name: {};", val),
         Prop::AnimationPlayState(val) => format!("animation-play-state: {};", val),
         Prop::AnimationTimingFunction(val) => format!("animation-timing-function: {};", val),
         Prop::BackfaceVisibility(val) => format!("backface-visibility: {};", val),
         Prop::Background(val) => format!("background: {};", val),
         Prop::BackgroundAttachment(val) => format!("background-attachment: {};", val),
         Prop::BackgroundBlendMode(val) => format!("background-blend-mode: {};", val),
         Prop::BackgroundClip(val) => format!("background-clip: {};", val),
         Prop::BackgroundColor(val) => format!("background-color: {};", val),
         Prop::BackgroundImage(val) => format!("background-image: {};", val),
         Prop::BackgroundOrigin(val) => format!("background-origin: {};", val),
         Prop::BackgroundPosition(val) => format!("background-position: {};", val),
         Prop::BackgroundRepeat(val) => format!("background-repeat: {};", val),
         Prop::BackgroundSize(val) => format!("background-size: {};", val),
         Prop::Border(val) => format!("border: {};", val),
         Prop::BorderBottom(val) => format!("border-bottom: {};", val),
         Prop::BorderBottomColor(val) => format!("border-bottom-color: {};", val),
         Prop::BorderBottomLeftRadius(val) => format!("border-bottom-left-radius: {};", val),
         Prop::BorderBottomRightRadius(val) => format!("border-bottom-right-radius: {};", val),
         Prop::BorderBottomStyle(val) => format!("border-bottom-style: {};", val),
         Prop::BorderBottomWidth(val) => format!("border-bottom-width: {};", val),
         Prop::BorderCollapse(val) => format!("border-collapse: {};", val),
         Prop::BorderColor(val) => format!("border-color: {};", val),
         Prop::BorderImage(val) => format!("border-image: {};", val),
         Prop::BorderImageOutset(val) => format!("border-image-outset: {};", val),
         Prop::BorderImageRepeat(val) => format!("border-image-repeat: {};", val),
         Prop::BorderImageSlice(val) => format!("border-image-slice: {};", val),
         Prop::BorderImageSource(val) => format!("border-image-source: {};", val),
         Prop::BorderImageWidth(val) => format!("border-image-width: {};", val),
         Prop::BorderLeft(val) => format!("border-left: {};", val),
         Prop::BorderLeftColor(val) => format!("border-left-color: {};", val),
         Prop::BorderLeftStyle(val) => format!("border-left-style: {};", val),
         Prop::BorderLeftWidth(val) => format!("border-left-width: {};", val),
         Prop::BorderRadius(val) => format!("border-radius: {};", val),
         Prop::BorderRight(val) => format!("border-right: {};", val),
         Prop::BorderRightColor(val) => format!("border-right-color: {};", val),
         Prop::BorderRightStyle(val) => format!("border-right-style: {};", val),
         Prop::BorderRightWidth(val) => format!("border-right-width: {};", val),
         Prop::BorderSpacing(val) => format!("border-spacing: {};", val),
         Prop::BorderStyle(val) => format!("border-style: {};", val),
         Prop::BorderTop(val) => format!("border-top: {};", val),
         Prop::BorderTopColor(val) => format!("border-top-color: {};", val),
         Prop::BorderTopLeftRadius(val) => format!("border-top-left-radius: {};", val),
         Prop::BorderTopRightRadius(val) => format!("border-top-right-radius: {};", val),
         Prop::BorderTopStyle(val) => format!("border-top-style: {};", val),
         Prop::BorderTopWidth(val) => format!("border-top-width: {};", val),
         Prop::BorderWidth(val) => format!("border-width: {};", val),
         Prop::Bottom(val) => format!("bottom: {};", val),
         Prop::BoxDecorationBreak(val) => format!("box-decoration-break: {};", val),
         Prop::BoxShadow(val) => format!("box-shadow: {};", val),
         Prop::BoxSizing(val) => format!("box-sizing: {};", val),
         Prop::BreakAfter(val) => format!("break-after: {};", val),
         Prop::BreakBefore(val) => format!("break-before: {};", val),
         Prop::BreakInside(val) => format!("break-inside: {};", val),
         Prop::CaptionSide(val) => format!("caption-side: {};", val),
         Prop::CaretColor(val) => format!("caret-color: {};", val),
         Prop::Clear(val) => format!("clear: {};", val),
         Prop::Clip(val) => format!("clip: {};", val),
         Prop::Color(val) => format!("color: {};", val),
         Prop::ColumnCount(val) => format!("column-count: {};", val),
         Prop::ColumnFill(val) => format!("column-fill: {};", val),
         Prop::ColumnGap(val) => format!("column-gap: {};", val),
         Prop::ColumnRule(val) => format!("column-rule: {};", val),
         Prop::ColumnRuleColor(val) => format!("column-rule-color: {};", val),
         Prop::ColumnRuleStyle(val) => format!("column-rule-style: {};", val),
         Prop::ColumnRuleWidth(val) => format!("column-rule-width: {};", val),
         Prop::ColumnSpan(val) => format!("column-span: {};", val),
         Prop::ColumnWidth(val) => format!("column-width: {};", val),
         Prop::Columns(val) => format!("columns: {};", val),
         Prop::Content(val) => format!("content: {};", val),
         Prop::CounterIncrement(val) => format!("counter-increment: {};", val),
         Prop::CounterReset(val) => format!("counter-reset: {};", val),
         Prop::Cursor(val) => format!("cursor: {};", val),
         Prop::Direction(val) => format!("direction: {};", val),
         Prop::Display(val) => format!("display: {};", val),
         Prop::EmptyCells(val) => format!("empty-cells: {};", val),
         Prop::Filter(val) => format!("filter: {};", val),
         Prop::Flex(val) => format!("flex: {};", val),
         Prop::FlexBasis(val) => format!("flex-basis: {};", val),
         Prop::FlexDirection(val) => format!("flex-direction: {};", val),
         Prop::FlexFlow(val) => format!("flex-flow: {};", val),
         Prop::FlexGrow(val) => format!("flex-grow: {};", val),
         Prop::FlexShrink(val) => format!("flex-shrink: {};", val),
         Prop::FlexWrap(val) => format!("flex-wrap: {};", val),
         Prop::Float(val) => format!("float: {};", val),
         Prop::Font(val) => format!("font: {};", val),
         Prop::FontFamily(val) => format!("font-family: {};", val),
         Prop::FontFeatureSettings(val) => format!("font-feature-settings: {};", val),
         Prop::FontKerning(val) => format!("font-kerning: {};", val),
         Prop::FontLanguageOverride(val) => format!("font-language-override: {};", val),
         Prop::FontSize(val) => format!("font-size: {};", val),
         Prop::FontSizeAdjust(val) => format!("font-size-adjust: {};", val),
         Prop::FontStretch(val) => format!("font-stretch: {};", val),
         Prop::FontStyle(val) => format!("font-style: {};", val),
         Prop::FontSynthesis(val) => format!("font-synthesis: {};", val),
         Prop::FontVariant(val) => format!("font-variant: {};", val),
         Prop::FontVariantAlternates(val) => format!("font-variant-alternates: {};", val),
         Prop::FontVariantCaps(val) => format!("font-variant-caps: {};", val),
         Prop::FontVariantEastAsian(val) => format!("font-variant-east-asian: {};", val),
         Prop::FontVariantLigatures(val) => format!("font-variant-ligatures: {};", val),
         Prop::FontVariantNumeric(val) => format!("font-variant-numeric: {};", val),
         Prop::FontVariantPosition(val) => format!("font-variant-position: {};", val),
         Prop::FontWeight(val) => format!("font-weight: {};", val),
         Prop::Grid(val) => format!("grid: {};", val),
         Prop::GridArea(val) => format!("grid-area: {};", val),
         Prop::GridAutoColumns(val) => format!("grid-auto-columns: {};", val),
         Prop::GridAutoFlow(val) => format!("grid-auto-flow: {};", val),
         Prop::GridAutoRows(val) => format!("grid-auto-rows: {};", val),
         Prop::GridColumn(val) => format!("grid-column: {};", val),
         Prop::GridColumnEnd(val) => format!("grid-column-end: {};", val),
         Prop::GridColumnGap(val) => format!("grid-column-gap: {};", val),
         Prop::GridColumnStart(val) => format!("grid-column-start: {};", val),
         Prop::GridGap(val) => format!("grid-gap: {};", val),
         Prop::GridRow(val) => format!("grid-row: {};", val),
         Prop::GridRowEnd(val) => format!("grid-row-end: {};", val),
         Prop::GridRowGap(val) => format!("grid-row-gap: {};", val),
         Prop::GridRowStart(val) => format!("grid-row-start: {};", val),
         Prop::GridTemplate(val) => format!("grid-template: {};", val),
         Prop::GridTemplateAreas(val) => format!("grid-template-areas: {};", val),
         Prop::GridTemplateColumns(val) => format!("grid-template-columns: {};", val),
         Prop::GridTemplateRows(val) => format!("grid-template-rows: {};", val),
         Prop::HangingPunctuation(val) => format!("hanging-punctuation: {};", val),
         Prop::Height(val) => format!("height: {};", val),
         Prop::Hyphens(val) => format!("hyphens: {};", val),
         Prop::ImageRendering(val) => format!("image-rendering: {};", val),
         Prop::Isolation(val) => format!("isolation: {};", val),
         Prop::JustifyContent(val) => format!("justify-content: {};", val),
         Prop::Left(val) => format!("left: {};", val),
         Prop::LetterSpacing(val) => format!("letter-spacing: {};", val),
         Prop::LineBreak(val) => format!("line-break: {};", val),
         Prop::LineHeight(val) => format!("line-height: {};", val),
         Prop::ListStyle(val) => format!("list-style: {};", val),
         Prop::ListStyleImage(val) => format!("list-style-image: {};", val),
         Prop::ListStylePosition(val) => format!("list-style-position: {};", val),
         Prop::ListStyleType(val) => format!("list-style-type: {};", val),
         Prop::Margin(val) => format!("margin: {};", val),
         Prop::MarginBottom(val) => format!("margin-bottom: {};", val),
         Prop::MarginLeft(val) => format!("margin-left: {};", val),
         Prop::MarginRight(val) => format!("margin-right: {};", val),
         Prop::MarginTop(val) => format!("margin-top: {};", val),
         Prop::MaxHeight(val) => format!("max-height: {};", val),
         Prop::MaxWidth(val) => format!("max-width: {};", val),
         Prop::MinHeight(val) => format!("min-height: {};", val),
         Prop::MinWidth(val) => format!("min-width: {};", val),
         Prop::MixBlendMode(val) => format!("mix-blend-mode: {};", val),
         Prop::ObjectFit(val) => format!("object-fit: {};", val),
         Prop::ObjectPosition(val) => format!("object-position: {};", val),
         Prop::Opacity(val) => format!("opacity: {};", val),
         Prop::Order(val) => format!("order: {};", val),
         Prop::Orphans(val) => format!("orphans: {};", val),
         Prop::Outline(val) => format!("outline: {};", val),
         Prop::OutlineColor(val) => format!("outline-color: {};", val),
         Prop::OutlineOffset(val) => format!("outline-offset: {};", val),
         Prop::OutlineStyle(val) => format!("outline-style: {};", val),
         Prop::OutlineWidth(val) => format!("outline-width: {};", val),
         Prop::Overflow(val) => format!("overflow: {};", val),
         Prop::OverflowWrap(val) => format!("overflow-wrap: {};", val),
         Prop::OverflowX(val) => format!("overflow-x: {};", val),
         Prop::OverflowY(val) => format!("overflow-y: {};", val),
         Prop::Padding(val) => format!("padding: {};", val),
         Prop::PaddingBottom(val) => format!("padding-bottom: {};", val),
         Prop::PaddingLeft(val) => format!("padding-left: {};", val),
         Prop::PaddingRight(val) => format!("padding-right: {};", val),
         Prop::PaddingTop(val) => format!("padding-top: {};", val),
         Prop::PageBreakAfter(val) => format!("page-break-after: {};", val),
         Prop::PageBreakBefore(val) => format!("page-break-before: {};", val),
         Prop::PageBreakInside(val) => format!("page-break-inside: {};", val),
         Prop::Perspective(val) => format!("perspective: {};", val),
         Prop::PerspectiveOrigin(val) => format!("perspective-origin: {};", val),
         Prop::PointerEvents(val) => format!("pointer-events: {};", val),
         Prop::Position(val) => format!("position: {};", val),
         Prop::Quotes(val) => format!("quotes: {};", val),
         Prop::Resize(val) => format!("resize: {};", val),
         Prop::Right(val) => format!("right: {};", val),
         Prop::ScrollBehavior(val) => format!("scroll-behavior: {};", val),
         Prop::TabSize(val) => format!("tab-size: {};", val),
         Prop::TableLayout(val) => format!("table-layout: {};", val),
         Prop::TextAlign(val) => format!("text-align: {};", val),
         Prop::TextAlignLast(val) => format!("text-align-last: {};", val),
         Prop::TextCombineUpright(val) => format!("text-combine-upright: {};", val),
         Prop::TextDecoration(val) => format!("text-decoration: {};", val),
         Prop::TextDecorationColor(val) => format!("text-decoration-color: {};", val),
         Prop::TextDecorationLine(val) => format!("text-decoration-line: {};", val),
         Prop::TextDecorationStyle(val) => format!("text-decoration-style: {};", val),
         Prop::TextIndent(val) => format!("text-indent: {};", val),
         Prop::TextJustify(val) => format!("text-justify: {};", val),
         Prop::TextOrientation(val) => format!("text-orientation: {};", val),
         Prop::TextOverflow(val) => format!("text-overflow: {};", val),
         Prop::TextShadow(val) => format!("text-shadow: {};", val),
         Prop::TextTransform(val) => format!("text-transform: {};", val),
         Prop::TextUnderlinePosition(val) => format!("text-underline-position: {};", val),
         Prop::Top(val) => format!("top: {};", val),
         Prop::Transform(val) => format!("transform: {};", val),
         Prop::TransformOrigin(val) => format!("transform-origin: {};", val),
         Prop::TransformStyle(val) => format!("transform-style: {};", val),
         Prop::Transition(val) => format!("transition: {};", val),
         Prop::TransitionDelay(val) => format!("transition-delay: {};", val),
         Prop::TransitionDuration(val) => format!("transition-duration: {};", val),
         Prop::TransitionProperty(val) => format!("transition-property: {};", val),
         Prop::TransitionTimingFunction(val) => format!("transition-timing-function: {};", val),
         Prop::UnicodeBidi(val) => format!("unicode-bidi: {};", val),
         Prop::UserSelect(val) => format!("user-select: {};", val),
         Prop::VerticalAlign(val) => format!("vertical-align: {};", val),
         Prop::Visibility(val) => format!("visibility: {};", val),
         Prop::WhiteSpace(val) => format!("white-space: {};", val),
         Prop::Widows(val) => format!("widows: {};", val),
         Prop::Width(val) => format!("width: {};", val),
         Prop::WordBreak(val) => format!("word-break: {};", val),
         Prop::WordSpacing(val) => format!("word-spacing: {};", val),
         Prop::WordWrap(val) => format!("word-wrap: {};", val),
         Prop::WritingMode(val) => format!("writing-mode: {};", val),
         Prop::ZIndex(val) => format!("z-index: {};", val),
      }.into()
   }
}

impl std::fmt::Display for Prop {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.into_string() )
   }
}